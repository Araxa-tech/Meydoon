use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use serde_json::json;

use crate::{
    entity::gig_image,
    error::AppError,
    handlers::dto::require_gig,
    middleware::AuthUser,
    state::AppState,
};

#[derive(Deserialize)]
pub struct AddImageRequest {
    pub image_url: String,
    #[serde(default)] pub sort_order: Option<i32>,
}

pub async fn add(
    State(state): State<AppState>,
    Path(gig_id): Path<u32>,
    auth: AuthUser,
    Json(req): Json<AddImageRequest>,
) -> Result<impl IntoResponse, AppError> {
    let gig = require_gig(&state.db, gig_id).await?;
    if gig.user_id != auth.id {
        return Err(AppError::Forbidden("not the gig owner".into()));
    }
    if req.image_url.trim().is_empty() {
        return Err(AppError::Validation("image_url: required".into()));
    }

    let next_order = req.sort_order.unwrap_or_else(|| {
        // naive: put at end
        1_000_000
    });

    let row = gig_image::ActiveModel {
        gig_id: Set(gig_id),
        image_url: Set(req.image_url.trim().to_string()),
        sort_order: Set(next_order),
        ..Default::default()
    }
    .insert(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(json!({ "id": row.id }))))
}

pub async fn remove(
    State(state): State<AppState>,
    Path((gig_id, image_id)): Path<(u32, u32)>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let gig = require_gig(&state.db, gig_id).await?;
    if gig.user_id != auth.id {
        return Err(AppError::Forbidden("not the gig owner".into()));
    }
    let res = gig_image::Entity::delete_many()
        .filter(gig_image::Column::Id.eq(image_id))
        .filter(gig_image::Column::GigId.eq(gig_id))
        .exec(&state.db)
        .await?;
    if res.rows_affected == 0 {
        return Err(AppError::NotFound("image not found".into()));
    }
    Ok(StatusCode::NO_CONTENT)
}
