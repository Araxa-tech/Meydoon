use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    entity::{
        gig::{self, GigStatus},
        gig_bid::{self, BidStatus},
        order::{self, OrderStatus},
    },
    error::AppError,
    handlers::dto::{fetch_users_map, BidDto, UserBrief},
    middleware::AuthUser,
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateBidRequest {
    pub amount_cents: i64,
    pub delivery_days: u32,
    pub message: String,
}

#[derive(Deserialize)]
pub struct UpdateBidRequest {
    pub amount_cents: Option<i64>,
    pub delivery_days: Option<u32>,
    pub message: Option<String>,
}

// ---------- list ----------

pub async fn list(
    State(state): State<AppState>,
    Path(gig_id): Path<u32>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let gig = gig::Entity::find_by_id(gig_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("gig not found".into()))?;

    let is_owner = gig.user_id == auth.id;

    let mut find = gig_bid::Entity::find()
        .filter(gig_bid::Column::GigId.eq(gig_id))
        .order_by_desc(gig_bid::Column::CreatedAt);
    if !is_owner {
        find = find.filter(gig_bid::Column::BidderId.eq(auth.id));
    }
    let bids = find.all(&state.db).await?;

    let ids: Vec<_> = bids.iter().map(|b| b.bidder_id).collect();
    let users = fetch_users_map(&state.db, &ids).await?;

    let items: Vec<BidDto> = bids
        .into_iter()
        .map(|b| {
            let brief = users.get(&b.bidder_id).cloned();
            BidDto::from_model(b, brief)
        })
        .collect();

    Ok(Json(json!({ "items": items })))
}

// ---------- create ----------

pub async fn create(
    State(state): State<AppState>,
    Path(gig_id): Path<u32>,
    auth: AuthUser,
    Json(req): Json<CreateBidRequest>,
) -> Result<impl IntoResponse, AppError> {
    let gig = gig::Entity::find_by_id(gig_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("gig not found".into()))?;

    if gig.user_id == auth.id {
        return Err(AppError::Forbidden("you cannot bid on your own gig".into()));
    }
    if gig.status != GigStatus::Open {
        return Err(AppError::BadRequest("gig is not open for bids".into()));
    }
    if req.amount_cents <= 0 {
        return Err(AppError::Validation("amount_cents must be > 0".into()));
    }
    if req.delivery_days == 0 {
        return Err(AppError::Validation("delivery_days must be > 0".into()));
    }
    if req.message.trim().is_empty() {
        return Err(AppError::Validation("message: required".into()));
    }

    // one bid per (user, gig)
    let existing = gig_bid::Entity::find()
        .filter(gig_bid::Column::GigId.eq(gig_id))
        .filter(gig_bid::Column::BidderId.eq(auth.id))
        .one(&state.db)
        .await?;
    if existing.is_some() {
        return Err(AppError::Conflict("you already bid on this gig".into()));
    }

    let now = Utc::now();
    let row = gig_bid::ActiveModel {
        gig_id: Set(gig_id),
        bidder_id: Set(auth.id),
        amount_cents: Set(req.amount_cents),
        delivery_days: Set(req.delivery_days),
        message: Set(req.message.trim().to_string()),
        status: Set(BidStatus::Pending),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(json!({ "id": row.id }))))
}

// ---------- update ----------

pub async fn update(
    State(state): State<AppState>,
    Path((gig_id, bid_id)): Path<(u32, u32)>,
    auth: AuthUser,
    Json(req): Json<UpdateBidRequest>,
) -> Result<impl IntoResponse, AppError> {
    let bid = gig_bid::Entity::find_by_id(bid_id)
        .filter(gig_bid::Column::GigId.eq(gig_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("bid not found".into()))?;

    if bid.bidder_id != auth.id {
        return Err(AppError::Forbidden("not the bid owner".into()));
    }
    if bid.status != BidStatus::Pending {
        return Err(AppError::BadRequest("only pending bids can be updated".into()));
    }

    let mut active: gig_bid::ActiveModel = bid.into();
    if let Some(a) = req.amount_cents {
        if a <= 0 { return Err(AppError::Validation("amount_cents must be > 0".into())); }
        active.amount_cents = Set(a);
    }
    if let Some(d) = req.delivery_days {
        if d == 0 { return Err(AppError::Validation("delivery_days must be > 0".into())); }
        active.delivery_days = Set(d);
    }
    if let Some(m) = req.message {
        if m.trim().is_empty() { return Err(AppError::Validation("message: required".into())); }
        active.message = Set(m.trim().to_string());
    }
    active.updated_at = Set(Utc::now());
    active.update(&state.db).await?;

    Ok(Json(json!({ "message": "updated" })))
}

// ---------- withdraw ----------

pub async fn withdraw(
    State(state): State<AppState>,
    Path((gig_id, bid_id)): Path<(u32, u32)>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let bid = gig_bid::Entity::find_by_id(bid_id)
        .filter(gig_bid::Column::GigId.eq(gig_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("bid not found".into()))?;

    if bid.bidder_id != auth.id {
        return Err(AppError::Forbidden("not the bid owner".into()));
    }
    if bid.status != BidStatus::Pending {
        return Err(AppError::BadRequest("only pending bids can be withdrawn".into()));
    }

    let mut active: gig_bid::ActiveModel = bid.into();
    active.status = Set(BidStatus::Withdrawn);
    active.updated_at = Set(Utc::now());
    active.update(&state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}

// ---------- accept ----------

pub async fn accept(
    State(state): State<AppState>,
    Path((gig_id, bid_id)): Path<(u32, u32)>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let txn = state.db.begin().await?;

    let gig = gig::Entity::find_by_id(gig_id)
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::NotFound("gig not found".into()))?;
    if gig.user_id != auth.id {
        return Err(AppError::Forbidden("only the gig owner can accept bids".into()));
    }
    if gig.status != GigStatus::Open {
        return Err(AppError::BadRequest("gig is not open".into()));
    }

    let bid = gig_bid::Entity::find_by_id(bid_id)
        .filter(gig_bid::Column::GigId.eq(gig_id))
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::NotFound("bid not found".into()))?;
    if bid.status != BidStatus::Pending {
        return Err(AppError::BadRequest("bid is no longer pending".into()));
    }

    let now = Utc::now();

    // 1. accept the winning bid
    gig_bid::ActiveModel {
        id: Set(bid.id),
        status: Set(BidStatus::Accepted),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(&txn)
    .await?;

    // 2. reject every other pending bid on this gig
    gig_bid::Entity::update_many()
        .col_expr(
            gig_bid::Column::Status,
            sea_orm::sea_query::Expr::value(BidStatus::Rejected),
        )
        .col_expr(
            gig_bid::Column::UpdatedAt,
            sea_orm::sea_query::Expr::value(now),
        )
        .filter(gig_bid::Column::GigId.eq(gig_id))
        .filter(gig_bid::Column::Id.ne(bid.id))
        .filter(gig_bid::Column::Status.eq(BidStatus::Pending))
        .exec(&txn)
        .await?;

    // 3. create the order
    let created = order::ActiveModel {
        bid_id: Set(Some(bid.id)),
        gig_id: Set(gig_id),
        client_id: Set(gig.user_id),
        freelancer_id: Set(bid.bidder_id),
        amount_cents: Set(bid.amount_cents),
        delivery_days: Set(bid.delivery_days),
        status: Set(OrderStatus::Active),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    // 4. gig → in_progress
    gig::ActiveModel {
        id: Set(gig_id),
        status: Set(GigStatus::InProgress),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(&txn)
    .await?;

    txn.commit().await?;

    Ok(Json(json!({
        "order_id": created.id,
        "message": "bid accepted"
    })))
}

// ---------- reject ----------

pub async fn reject(
    State(state): State<AppState>,
    Path((gig_id, bid_id)): Path<(u32, u32)>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let gig = gig::Entity::find_by_id(gig_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("gig not found".into()))?;
    if gig.user_id != auth.id {
        return Err(AppError::Forbidden("only the gig owner can reject bids".into()));
    }

    let bid = gig_bid::Entity::find_by_id(bid_id)
        .filter(gig_bid::Column::GigId.eq(gig_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("bid not found".into()))?;
    if bid.status != BidStatus::Pending {
        return Err(AppError::BadRequest("only pending bids can be rejected".into()));
    }

    gig_bid::ActiveModel {
        id: Set(bid.id),
        status: Set(BidStatus::Rejected),
        updated_at: Set(Utc::now()),
        ..Default::default()
    }
    .update(&state.db)
    .await?;

    Ok(Json(json!({ "message": "rejected" })))
}
