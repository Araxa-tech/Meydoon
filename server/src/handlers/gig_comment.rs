use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set, TransactionTrait};
use serde::Deserialize;
use serde_json::json;

use crate::{
    entity::{gig, gig_comment},
    error::AppError,
    handlers::dto::{build_comment_tree, require_gig, CommentDto},
    middleware::AuthUser,
    state::AppState,
};

#[derive(Deserialize)]
pub struct CommentRequest {
    pub body: String,
}

pub async fn list(
    State(state): State<AppState>,
    Path(gig_id): Path<u32>,
) -> Result<impl IntoResponse, AppError> {
    let _ = require_gig(&state.db, gig_id).await?;
    let comments = gig_comment::Entity::find()
        .filter(gig_comment::Column::GigId.eq(gig_id))
        .order_by_asc(gig_comment::Column::CreatedAt)
        .all(&state.db)
        .await?;
    let tree: Vec<CommentDto> = build_comment_tree(&state.db, comments).await?;
    Ok(Json(json!({ "items": tree })))
}

pub async fn create(
    State(state): State<AppState>,
    Path(gig_id): Path<u32>,
    auth: AuthUser,
    Json(req): Json<CommentRequest>,
) -> Result<impl IntoResponse, AppError> {
    let _ = require_gig(&state.db, gig_id).await?;
    if req.body.trim().is_empty() {
        return Err(AppError::Validation("body: required".into()));
    }
    if req.body.len() > 2000 {
        return Err(AppError::Validation("body: ≤2000 chars".into()));
    }

    let now = Utc::now();
    let row = gig_comment::ActiveModel {
        gig_id: Set(gig_id),
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
    Path((gig_id, cid)): Path<(u32, u32)>,
    auth: AuthUser,
    Json(req): Json<CommentRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.body.trim().is_empty() {
        return Err(AppError::Validation("body: required".into()));
    }
    let c = gig_comment::Entity::find_by_id(cid)
        .filter(gig_comment::Column::GigId.eq(gig_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("comment not found".into()))?;
    if c.user_id != auth.id {
        return Err(AppError::Forbidden("not the comment author".into()));
    }
    let mut active: gig_comment::ActiveModel = c.into();
    active.body = Set(req.body.trim().to_string());
    active.updated_at = Set(Utc::now());
    active.update(&state.db).await?;
    Ok(Json(json!({ "message": "updated" })))
}

pub async fn delete(
    State(state): State<AppState>,
    Path((gig_id, cid)): Path<(u32, u32)>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let c = gig_comment::Entity::find_by_id(cid)
        .filter(gig_comment::Column::GigId.eq(gig_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("comment not found".into()))?;

    // Allow the comment author OR the gig owner (moderation).
    let gig = gig::Entity::find_by_id(gig_id).one(&state.db).await?
        .ok_or_else(|| AppError::NotFound("gig not found".into()))?;
    if c.user_id != auth.id && gig.user_id != auth.id {
        return Err(AppError::Forbidden("not allowed".into()));
    }

    use crate::entity::gig_comment_reply;
    let txn = state.db.begin().await?;
    gig_comment_reply::Entity::delete_many()
        .filter(gig_comment_reply::Column::CommentId.eq(cid))
        .exec(&txn).await?;
    gig_comment::Entity::delete_by_id(cid).exec(&txn).await?;
    txn.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}
