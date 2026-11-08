use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use serde_json::json;

use crate::{
    entity::gig_rating::{self, RatingValue},
    error::AppError,
    handlers::dto::require_gig,
    middleware::AuthUser,
    state::AppState,
};

#[derive(Deserialize)]
pub struct UpsertRatingRequest {
    /// 1 = upvote, -1 = downvote. Anything else is rejected.
    pub value: i32,
}

pub async fn upsert(
    State(state): State<AppState>,
    Path(gig_id): Path<u32>,
    auth: AuthUser,
    Json(req): Json<UpsertRatingRequest>,
) -> Result<impl IntoResponse, AppError> {
    let _gig = require_gig(&state.db, gig_id).await?;
    let value = match req.value {
        1 => RatingValue::Upvote,
        -1 => RatingValue::Downvote,
        _ => return Err(AppError::Validation("value must be 1 or -1".into())),
    };

    let existing = gig_rating::Entity::find()
        .filter(gig_rating::Column::GigId.eq(gig_id))
        .filter(gig_rating::Column::UserId.eq(auth.id))
        .one(&state.db)
        .await?;

    match existing {
        Some(r) => {
            let mut active: gig_rating::ActiveModel = r.into();
            active.value = Set(value);
            active.updated_at = Set(Utc::now());
            active.update(&state.db).await?;
        }
        None => {
            gig_rating::ActiveModel {
                gig_id: Set(gig_id),
                user_id: Set(auth.id),
                value: Set(value),
                created_at: Set(Utc::now()),
                updated_at: Set(Utc::now()),
                ..Default::default()
            }
            .insert(&state.db)
            .await?;
        }
    }
    Ok(Json(json!({ "message": "recorded" })))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(gig_id): Path<u32>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let res = gig_rating::Entity::delete_many()
        .filter(gig_rating::Column::GigId.eq(gig_id))
        .filter(gig_rating::Column::UserId.eq(auth.id))
        .exec(&state.db)
        .await?;
    if res.rows_affected == 0 {
        return Err(AppError::NotFound("no rating to remove".into()));
    }
    Ok(StatusCode::NO_CONTENT)
}
