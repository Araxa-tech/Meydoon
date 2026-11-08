use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::Deserialize;
use serde_json::json;

use crate::{
    entity::{gig, gig_comment, gig_comment_reply},
    error::AppError,
    handlers::dto::{fetch_users_map, ReplyDto, UserBrief},
    middleware::AuthUser,
    state::AppState,
};

#[derive(Deserialize)]
pub struct ReplyRequest {
    pub body: String,
}

/// Look up (comment, gig). Ensures the comment really belongs to the gig.
async fn load_comment_and_gig(
    state: &AppState,
    gig_id: u32,
    comment_id: u32,
) -> Result<(gig_comment::Model, gig::Model), AppError> {
    let comment = gig_comment::Entity::find_by_id(comment_id)
        .filter(gig_comment::Column::GigId.eq(gig_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("comment not found".into()))?;
    let gig = gig::Entity::find_by_id(gig_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("gig not found".into()))?;
    Ok((comment, gig))
}

pub async fn list(
    State(state): State<AppState>,
    Path((gig_id, cid)): Path<(u32, u32)>,
) -> Result<impl IntoResponse, AppError> {
    let _ = load_comment_and_gig(&state, gig_id, cid).await?;
    let replies = gig_comment_reply::Entity::find()
        .filter(gig_comment_reply::Column::CommentId.eq(cid))
        .order_by_asc(gig_comment_reply::Column::CreatedAt)
        .all(&state.db)
        .await?;
    let user_ids: Vec<_> = replies.iter().map(|r| r.user_id).collect();
    let users = fetch_users_map(&state.db, &user_ids).await?;
    let dtos: Vec<ReplyDto> = replies.into_iter().map(|r| {
        let author = users.get(&r.user_id).cloned().unwrap_or_else(|| UserBrief::unknown(r.user_id));
        ReplyDto {
            id: r.id, user_id: r.user_id, body: r.body,
            created_at: r.created_at, updated_at: r.updated_at,
            author,
        }
    }).collect();
    Ok(Json(json!({ "items": dtos })))
}

pub async fn create(
    State(state): State<AppState>,
    Path((gig_id, cid)): Path<(u32, u32)>,
    auth: AuthUser,
    Json(req): Json<ReplyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (_comment, gig) = load_comment_and_gig(&state, gig_id, cid).await?;

    // ---- the rule: only the gig creator can reply ----
    if gig.user_id != auth.id {
        return Err(AppError::Forbidden(
            "only the gig creator can reply to comments".into(),
        ));
    }

    if req.body.trim().is_empty() {
        return Err(AppError::Validation("body: required".into()));
    }
    if req.body.len() > 2000 {
        return Err(AppError::Validation("body: ≤2000 chars".into()));
    }

    let now = Utc::now();
    let row = gig_comment_reply::ActiveModel {
        comment_id: Set(cid),
        user_id: Set(auth.id),
        body: Set(req.body.trim().to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(json!({ "id": row.id }))))
}

pub async fn update(
    State(state): State<AppState>,
    Path((gig_id, cid, rid)): Path<(u32, u32, u32)>,
    auth: AuthUser,
    Json(req): Json<ReplyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let _ = load_comment_and_gig(&state, gig_id, cid).await?;
    if req.body.trim().is_empty() {
        return Err(AppError::Validation("body: required".into()));
    }
    let r = gig_comment_reply::Entity::find_by_id(rid)
        .filter(gig_comment_reply::Column::CommentId.eq(cid))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("reply not found".into()))?;
    if r.user_id != auth.id {
        return Err(AppError::Forbidden("not the reply author".into()));
    }
    let mut active: gig_comment_reply::ActiveModel = r.into();
    active.body = Set(req.body.trim().to_string());
    active.updated_at = Set(Utc::now());
    active.update(&state.db).await?;
    Ok(Json(json!({ "message": "updated" })))
}

pub async fn delete(
    State(state): State<AppState>,
    Path((gig_id, cid, rid)): Path<(u32, u32, u32)>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let _ = load_comment_and_gig(&state, gig_id, cid).await?;
    let r = gig_comment_reply::Entity::find_by_id(rid)
        .filter(gig_comment_reply::Column::CommentId.eq(cid))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("reply not found".into()))?;
    if r.user_id != auth.id {
        return Err(AppError::Forbidden("not the reply author".into()));
    }
    gig_comment_reply::Entity::delete_by_id(rid).exec(&state.db).await?;
    Ok(StatusCode::NO_CONTENT)
}
