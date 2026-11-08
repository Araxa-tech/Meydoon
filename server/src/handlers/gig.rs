use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait,
};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

use crate::{
    entity::{gig::{self, GigStatus, TagList}, gig_bid, gig_category, gig_comment, gig_image, gig_rating, order::{self, OrderStatus}},
    error::AppError,
    handlers::dto::{
        BidDto, GigDetailDto, GigSummaryDto, ImageDto, OrderSummaryDto, RatingSummary, UserBrief, build_comment_tree, fetch_users_map, require_gig
    },
    middleware::{AuthUser, OptionalAuthUser},
    state::AppState,
};

// ---------- payloads ----------

#[derive(Deserialize)]
pub struct ListQuery {
    #[serde(default)] pub page: Option<u64>,
    #[serde(default)] pub per_page: Option<u64>,
    #[serde(default)] pub category_id: Option<u32>,
}

#[derive(Deserialize)]
pub struct CreateGigRequest {
    pub title: String,
    pub description: String,
    pub category_id: u32,
    #[serde(default)] pub tags: Vec<String>,
    /// URLs / paths. Images are rows in gig_images, files live on disk / S3.
    #[serde(default)] pub images: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdateGigRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<u32>,
    pub tags: Option<Vec<String>>,
}

// ---------- handlers ----------

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let per_page = q.per_page.unwrap_or(20).clamp(1, 100);
    let page = q.page.unwrap_or(1).max(1);

    let mut find = gig::Entity::find().order_by_desc(gig::Column::CreatedAt);
    if let Some(cid) = q.category_id {
        find = find.filter(gig::Column::CategoryId.eq(cid));
    }

    let paginator = find.paginate(&state.db, per_page);
    let total = paginator.num_items().await?;
    let gigs = paginator.fetch_page(page - 1).await?;

    if gigs.is_empty() {
        return Ok(Json(json!({
            "items": [], "page": page, "per_page": per_page, "total": total
        })));
    }

    let gig_ids: Vec<u32> = gigs.iter().map(|g| g.id).collect();
    let user_ids: Vec<_> = gigs.iter().map(|g| g.user_id).collect();
    let users = fetch_users_map(&state.db, &user_ids).await?;

    // primary image per gig
    let images = gig_image::Entity::find()
        .filter(gig_image::Column::GigId.is_in(gig_ids.clone()))
        .order_by_asc(gig_image::Column::SortOrder)
        .all(&state.db)
        .await?;
    let mut primary_image: HashMap<u32, String> = HashMap::new();
    for img in images {
        primary_image.entry(img.gig_id).or_insert(img.image_url);
    }

    // rating aggregate per gig
    let ratings = gig_rating::Entity::find()
        .filter(gig_rating::Column::GigId.is_in(gig_ids.clone()))
        .all(&state.db)
        .await?;
    let mut scores: HashMap<u32, RatingSummary> = HashMap::new();
    for r in ratings {
        let e = scores.entry(r.gig_id).or_default();
        match r.value {
            gig_rating::RatingValue::Upvote => { e.upvotes += 1; e.score += 1; }
            gig_rating::RatingValue::Downvote => { e.downvotes += 1; e.score -= 1; }
        }
    }

    // comment count per gig
    let counts = gig_comment::Entity::find()
        .select_only()
        .column(gig_comment::Column::GigId)
        .column_as(gig_comment::Column::Id.count(), "count")
        .filter(gig_comment::Column::GigId.is_in(gig_ids))
        .group_by(gig_comment::Column::GigId)
        .into_tuple::<(u32, i64)>()
        .all(&state.db)
        .await?;
    let comment_count: HashMap<u32, i64> = counts.into_iter().collect();

    let items: Vec<GigSummaryDto> = gigs.into_iter().map(|g| {
        let author = users.get(&g.user_id).cloned().unwrap_or_else(|| UserBrief::unknown(g.user_id));
        GigSummaryDto {
            id: g.id, title: g.title, user_id: g.user_id, category_id: g.category_id,
            tags: g.tags.0, created_at: g.created_at, updated_at: g.updated_at,
            author,
            primary_image: primary_image.get(&g.id).cloned(),
            score: scores.get(&g.id).map(|s| s.score).unwrap_or(0),
            comment_count: comment_count.get(&g.id).copied().unwrap_or(0),
        }
    }).collect();

    Ok(Json(json!({
        "items": items, "page": page, "per_page": per_page, "total": total
    })))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    auth: OptionalAuthUser,
) -> Result<impl IntoResponse, AppError> {
    let gig = require_gig(&state.db, id).await?;

    let author = fetch_users_map(&state.db, &[gig.user_id])
        .await?
        .get(&gig.user_id)
        .cloned()
        .unwrap_or_else(|| UserBrief::unknown(gig.user_id));

    let images = gig_image::Entity::find()
        .filter(gig_image::Column::GigId.eq(id))
        .order_by_asc(gig_image::Column::SortOrder)
        .all(&state.db)
        .await?;

    let ratings = gig_rating::Entity::find()
        .filter(gig_rating::Column::GigId.eq(id))
        .all(&state.db)
        .await?;
    let mut summary = RatingSummary::default();
    for r in &ratings {
        match r.value {
            gig_rating::RatingValue::Upvote => { summary.upvotes += 1; summary.score += 1; }
            gig_rating::RatingValue::Downvote => { summary.downvotes += 1; summary.score -= 1; }
        }
    }
    let my_rating = auth.0.as_ref().and_then(|u| {
        ratings.iter().find(|r| r.user_id == u.id).map(|r| match r.value {
            gig_rating::RatingValue::Upvote => 1,
            gig_rating::RatingValue::Downvote => -1,
        })
    });

    let comments = gig_comment::Entity::find()
        .filter(gig_comment::Column::GigId.eq(id))
        .order_by_asc(gig_comment::Column::CreatedAt)
        .all(&state.db)
        .await?;
    let comments = build_comment_tree(&state.db, comments).await?;

    // ---- bids + order for the authed viewer ----
    let (my_bid, bids, bid_count, my_order) = if let Some(u) = auth.0.as_ref() {
        let is_owner = gig.user_id == u.id;

        let bid_count = gig_bid::Entity::find()
            .filter(gig_bid::Column::GigId.eq(id))
            .count(&state.db)
            .await? as i64;

        let my_bid = gig_bid::Entity::find()
            .filter(gig_bid::Column::GigId.eq(id))
            .filter(gig_bid::Column::BidderId.eq(u.id))
            .one(&state.db)
            .await?
            .map(|b| BidDto::from_model(b, None));

        let bids = if is_owner {
            let rows = gig_bid::Entity::find()
                .filter(gig_bid::Column::GigId.eq(id))
                .order_by_desc(gig_bid::Column::CreatedAt)
                .all(&state.db)
                .await?;
            let ids: Vec<_> = rows.iter().map(|b| b.bidder_id).collect();
            let users = fetch_users_map(&state.db, &ids).await?;
            Some(
                rows.into_iter()
                    .map(|b| BidDto::from_model(b.clone(), users.get(&b.bidder_id).cloned()))
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        };

        let my_order = order::Entity::find()
            .filter(order::Column::GigId.eq(id))
            .filter(
                sea_orm::Condition::any()
                    .add(order::Column::ClientId.eq(u.id))
                    .add(order::Column::FreelancerId.eq(u.id)),
            )
            .one(&state.db)
            .await?
            .map(|o| OrderSummaryDto {
                id: o.id,
                status: o.status,
                amount_cents: o.amount_cents,
                client_id: o.client_id,
                freelancer_id: o.freelancer_id,
            });

        (my_bid, bids, bid_count, my_order)
    } else {
        let bid_count = gig_bid::Entity::find()
            .filter(gig_bid::Column::GigId.eq(id))
            .count(&state.db)
            .await? as i64;
        (None, None, bid_count, None)
    };

    Ok(Json(GigDetailDto {
        id: gig.id,
        title: gig.title,
        description: gig.description,
        user_id: gig.user_id,
        category_id: gig.category_id,
        tags: gig.tags.0,
        status: gig.status,
        budget_cents: gig.budget_cents,
        created_at: gig.created_at,
        updated_at: gig.updated_at,
        author,
        images: images.into_iter().map(ImageDto::from).collect(),
        rating: summary,
        my_rating,
        comments,
        bid_count,
        my_bid,
        bids,
        my_order,
    }))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateGigRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.title.trim().is_empty() || req.title.len() > 200 {
        return Err(AppError::Validation("title: required, ≤200 chars".into()));
    }
    if req.description.trim().is_empty() {
        return Err(AppError::Validation("description: required".into()));
    }
    if gig_category::Entity::find_by_id(req.category_id).one(&state.db).await?.is_none() {
        return Err(AppError::BadRequest("category_id does not exist".into()));
    }

    let txn = state.db.begin().await?;
    let now = Utc::now();
    let created = gig::ActiveModel {
        title: Set(req.title.trim().to_string()),
        description: Set(req.description.trim().to_string()),
        user_id: Set(auth.id),
        category_id: Set(req.category_id),
        created_at: Set(now),
        updated_at: Set(now),
        status: Set(GigStatus::Open),
        tags: Set(TagList(req.tags)),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    for (i, url) in req.images.iter().enumerate() {
        gig_image::ActiveModel {
            gig_id: Set(created.id),
            image_url: Set(url.clone()),
            sort_order: Set(i as i32),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
    }
    txn.commit().await?;

    Ok((StatusCode::CREATED, Json(json!({ "id": created.id }))))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    auth: AuthUser,
    Json(req): Json<UpdateGigRequest>,
) -> Result<impl IntoResponse, AppError> {
    let gig = require_gig(&state.db, id).await?;
    if gig.user_id != auth.id {
        return Err(AppError::Forbidden("not the gig owner".into()));
    }
    if gig.status != GigStatus::Open {
        return Err(AppError::BadRequest("only open gigs can be edited".into()));
    }

    let mut active: gig::ActiveModel = gig.into();
    if let Some(t) = req.title {
        if t.trim().is_empty() { return Err(AppError::Validation("title cannot be empty".into())); }
        active.title = Set(t);
    }
    if let Some(d) = req.description { active.description = Set(d); }
    if let Some(c) = req.category_id {
        if gig_category::Entity::find_by_id(c).one(&state.db).await?.is_none() {
            return Err(AppError::BadRequest("category_id does not exist".into()));
        }
        active.category_id = Set(c);
    }
    if let Some(t) = req.tags { active.tags = Set(TagList(t)); }
    active.updated_at = Set(Utc::now());
    active.update(&state.db).await?;

    Ok(Json(json!({ "message": "updated" })))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let gig = require_gig(&state.db, id).await?;
    if gig.user_id != auth.id {
        return Err(AppError::Forbidden("not the gig owner".into()));
    }
    let has_active = order::Entity::find()
        .filter(order::Column::GigId.eq(id))
        .filter(order::Column::Status.is_in([
                OrderStatus::Active,
                OrderStatus::Delivered,
                OrderStatus::Disputed,
        ]))
        .one(&state.db)
        .await?
        .is_some();
    if has_active {
        return Err(AppError::BadRequest("cannot delete a gig with an active order".into()));
    }
    let txn = state.db.begin().await?;

    // cascade: images, ratings, comments + replies
    gig_image::Entity::delete_many()
        .filter(gig_image::Column::GigId.eq(id)).exec(&txn).await?;
    gig_rating::Entity::delete_many()
        .filter(gig_rating::Column::GigId.eq(id)).exec(&txn).await?;

    let comment_ids: Vec<u32> = gig_comment::Entity::find()
        .select_only().column(gig_comment::Column::Id)
        .filter(gig_comment::Column::GigId.eq(id))
        .into_tuple::<u32>().all(&txn).await?;
    if !comment_ids.is_empty() {
        use crate::entity::gig_comment_reply;
        gig_comment_reply::Entity::delete_many()
            .filter(gig_comment_reply::Column::CommentId.is_in(comment_ids))
            .exec(&txn).await?;
    }
    gig_comment::Entity::delete_many()
        .filter(gig_comment::Column::GigId.eq(id)).exec(&txn).await?;

    gig::Entity::delete_by_id(id).exec(&txn).await?;
    txn.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}
