use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set, TransactionTrait,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{
    entity::{
        gig::{self, GigStatus},
        order::{self, OrderStatus, StringList},
        user,
    },
    error::AppError,
    handlers::dto::{OrderDto, OrderSummaryDto, UserBrief},
    middleware::AuthUser,
    state::AppState,
};

#[derive(Deserialize)]
pub struct ListQuery {
    /// "client" | "freelancer" | "both" (default)
    #[serde(default)] pub role: Option<String>,
    #[serde(default)] pub status: Option<String>,
    #[serde(default)] pub page: Option<u64>,
    #[serde(default)] pub per_page: Option<u64>,
}

#[derive(Deserialize)]
pub struct CancelRequest { pub reason: String }

#[derive(Deserialize)]
pub struct DeliverRequest {
    pub message: String,
    #[serde(default)] pub delivery_urls: Vec<String>,
}

#[derive(Deserialize)]
pub struct DisputeRequest { pub reason: String }

// ---------- helpers ----------

async fn load_order_for_user(
    state: &AppState,
    id: u32,
    user_id: Uuid,
) -> Result<order::Model, AppError> {
    let o = order::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("order not found".into()))?;
    if o.client_id != user_id && o.freelancer_id != user_id {
        return Err(AppError::Forbidden("not a party to this order".into()));
    }
    Ok(o)
}

async fn hydrate_order(state: &AppState, o: order::Model) -> Result<OrderDto, AppError> {
    let gig = gig::Entity::find_by_id(o.gig_id).one(&state.db).await?;
    let title = gig.map(|g| g.title).unwrap_or_default();

    let users = user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Id.eq(o.client_id))
                .add(user::Column::Id.eq(o.freelancer_id)),
        )
        .all(&state.db)
        .await?;
    let client = users.iter().find(|u| u.id == o.client_id).map(UserBrief::from);
    let freelancer = users.iter().find(|u| u.id == o.freelancer_id).map(UserBrief::from);

    Ok(OrderDto::from_model(o, title, client, freelancer))
}

// ---------- list ----------

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(q): Query<ListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let per_page = q.per_page.unwrap_or(20).clamp(1, 100);
    let page = q.page.unwrap_or(1).max(1);

    let user_filter = match q.role.as_deref() {
        Some("client") => Condition::all().add(order::Column::ClientId.eq(auth.id)),
        Some("freelancer") => Condition::all().add(order::Column::FreelancerId.eq(auth.id)),
        _ => Condition::any()
            .add(order::Column::ClientId.eq(auth.id))
            .add(order::Column::FreelancerId.eq(auth.id)),
    };

    let mut find = order::Entity::find()
        .filter(user_filter)
        .order_by_desc(order::Column::CreatedAt);

    if let Some(s) = q.status.as_deref() {
        if let Some(st) = status_from_str(s) {
            find = find.filter(order::Column::Status.eq(st));
        }
    }

    let paginator = find.paginate(&state.db, per_page);
    let total = paginator.num_items().await?;
    let rows = paginator.fetch_page(page - 1).await?;

    // hydrate in bulk
    let mut items = Vec::with_capacity(rows.len());
    for o in rows {
        items.push(hydrate_order(&state, o).await?);
    }

    Ok(Json(json!({
        "items": items, "page": page, "per_page": per_page, "total": total
    })))
}

fn status_from_str(s: &str) -> Option<OrderStatus> {
    Some(match s {
        "active" => OrderStatus::Active,
        "delivered" => OrderStatus::Delivered,
        "completed" => OrderStatus::Completed,
        "cancelled" => OrderStatus::Cancelled,
        "disputed" => OrderStatus::Disputed,
        _ => return None,
    })
}

// ---------- get ----------

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let o = load_order_for_user(&state, id, auth.id).await?;
    Ok(Json(hydrate_order(&state, o).await?))
}

// ---------- deliver ----------

pub async fn deliver(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    auth: AuthUser,
    Json(req): Json<DeliverRequest>,
) -> Result<impl IntoResponse, AppError> {
    let o = load_order_for_user(&state, id, auth.id).await?;
    if o.freelancer_id != auth.id {
        return Err(AppError::Forbidden("only the freelancer can deliver".into()));
    }
    if !o.status.can_transition_to(OrderStatus::Delivered) {
        return Err(AppError::BadRequest(format!("cannot deliver from {:?}", o.status)));
    }
    if req.message.trim().is_empty() {
        return Err(AppError::Validation("message: required".into()));
    }

    let now = Utc::now();
    let mut active: order::ActiveModel = o.into();
    active.status = Set(OrderStatus::Delivered);
    active.delivery_message = Set(Some(req.message.trim().to_string()));
    if !req.delivery_urls.is_empty() {
        active.delivery_urls = Set(Some(StringList(req.delivery_urls)));
    }
    active.delivered_at = Set(Some(now));
    active.updated_at = Set(now);
    active.update(&state.db).await?;

    Ok(Json(json!({ "message": "delivered" })))
}

// ---------- complete ----------

pub async fn complete(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let o = load_order_for_user(&state, id, auth.id).await?;
    if o.client_id != auth.id {
        return Err(AppError::Forbidden("only the client can complete".into()));
    }
    if !o.status.can_transition_to(OrderStatus::Completed) {
        return Err(AppError::BadRequest(format!("cannot complete from {:?}", o.status)));
    }

    let txn = state.db.begin().await?;
    let now = Utc::now();

    order::ActiveModel {
        id: Set(o.id),
        status: Set(OrderStatus::Completed),
        completed_at: Set(Some(now)),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(&txn)
    .await?;

    gig::ActiveModel {
        id: Set(o.gig_id),
        status: Set(GigStatus::Completed),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(&txn)
    .await?;

    txn.commit().await?;
    Ok(Json(json!({ "message": "completed" })))
}

// ---------- cancel ----------

pub async fn cancel(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    auth: AuthUser,
    Json(req): Json<CancelRequest>,
) -> Result<impl IntoResponse, AppError> {
    let o = load_order_for_user(&state, id, auth.id).await?;
    if !o.status.can_transition_to(OrderStatus::Cancelled) {
        return Err(AppError::BadRequest(format!("cannot cancel from {:?}", o.status)));
    }
    if req.reason.trim().is_empty() {
        return Err(AppError::Validation("reason: required".into()));
    }

    let txn = state.db.begin().await?;
    let now = Utc::now();

    order::ActiveModel {
        id: Set(o.id),
        status: Set(OrderStatus::Cancelled),
        cancelled_by: Set(Some(auth.id)),
        cancel_reason: Set(Some(req.reason.trim().to_string())),
        cancelled_at: Set(Some(now)),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(&txn)
    .await?;

    // Reopen the gig so it can receive new bids.
    gig::ActiveModel {
        id: Set(o.gig_id),
        status: Set(GigStatus::Open),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(&txn)
    .await?;

    txn.commit().await?;
    Ok(Json(json!({ "message": "cancelled" })))
}

// ---------- dispute ----------

pub async fn dispute(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    auth: AuthUser,
    Json(req): Json<DisputeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let o = load_order_for_user(&state, id, auth.id).await?;
    if !o.status.can_transition_to(OrderStatus::Disputed) {
        return Err(AppError::BadRequest(format!("cannot dispute from {:?}", o.status)));
    }
    if req.reason.trim().is_empty() {
        return Err(AppError::Validation("reason: required".into()));
    }

    order::ActiveModel {
        id: Set(o.id),
        status: Set(OrderStatus::Disputed),
        dispute_reason: Set(Some(req.reason.trim().to_string())),
        updated_at: Set(Utc::now()),
        ..Default::default()
    }
    .update(&state.db)
    .await?;

    Ok(Json(json!({ "message": "disputed" })))
}
