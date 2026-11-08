use std::collections::HashMap;
use chrono::DateTime;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    entity::{gig, gig_comment, gig_comment_reply, gig_image, user,
    gig_bid, order,},
    error::AppError,
};

pub type DateTimeUtc = DateTime<chrono::Utc>;


#[derive(Serialize)]
pub struct BidDto {
    pub id: u32,
    pub gig_id: u32,
    pub bidder_id: Uuid,
    pub amount_cents: i64,
    pub delivery_days: u32,
    pub message: String,
    pub status: gig_bid::BidStatus,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub bidder: Option<UserBrief>,
}

impl BidDto {
    pub fn from_model(b: gig_bid::Model, bidder: Option<UserBrief>) -> Self {
        Self {
            id: b.id,
            gig_id: b.gig_id,
            bidder_id: b.bidder_id,
            amount_cents: b.amount_cents,
            delivery_days: b.delivery_days,
            message: b.message,
            status: b.status,
            created_at: b.created_at,
            updated_at: b.updated_at,
            bidder,
        }
    }
}

#[derive(Serialize)]
pub struct OrderDto {
    pub id: u32,
    pub bid_id: Option<u32>,
    pub gig_id: u32,
    pub gig_title: String,
    pub client_id: Uuid,
    pub freelancer_id: Uuid,
    pub amount_cents: i64,
    pub delivery_days: u32,
    pub status: order::OrderStatus,
    pub delivery_message: Option<String>,
    pub delivery_urls: Option<Vec<String>>,
    pub cancel_reason: Option<String>,
    pub dispute_reason: Option<String>,
    pub delivered_at: Option<DateTimeUtc>,
    pub completed_at: Option<DateTimeUtc>,
    pub cancelled_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub client: Option<UserBrief>,
    pub freelancer: Option<UserBrief>,
}

impl OrderDto {
    pub fn from_model(
        o: order::Model,
        gig_title: String,
        client: Option<UserBrief>,
        freelancer: Option<UserBrief>,
    ) -> Self {
        Self {
            id: o.id,
            bid_id: o.bid_id,
            gig_id: o.gig_id,
            gig_title,
            client_id: o.client_id,
            freelancer_id: o.freelancer_id,
            amount_cents: o.amount_cents,
            delivery_days: o.delivery_days,
            status: o.status,
            delivery_message: o.delivery_message,
            delivery_urls: o.delivery_urls.map(|s| s.0),
            cancel_reason: o.cancel_reason,
            dispute_reason: o.dispute_reason,
            delivered_at: o.delivered_at,
            completed_at: o.completed_at,
            cancelled_at: o.cancelled_at,
            created_at: o.created_at,
            updated_at: o.updated_at,
            client,
            freelancer,
        }
    }
}

#[derive(Serialize)]
pub struct OrderSummaryDto {
    pub id: u32,
    pub status: order::OrderStatus,
    pub amount_cents: i64,
    pub client_id: Uuid,
    pub freelancer_id: Uuid,
}

#[derive(Serialize, Clone)]
pub struct UserBrief {
    pub id: Uuid,
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
}

impl UserBrief {
    pub fn unknown(id: Uuid) -> Self {
        Self { id, user_name: "[deleted]".into(), first_name: String::new(), last_name: String::new() }
    }
}

impl From<&user::Model> for UserBrief {
    fn from(u: &user::Model) -> Self {
        Self {
            id: u.id,
            user_name: u.user_name.clone(),
            first_name: u.first_name.clone(),
            last_name: u.last_name.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct ImageDto {
    pub id: u32,
    pub image_url: String,
    pub sort_order: i32,
}
impl From<gig_image::Model> for ImageDto {
    fn from(m: gig_image::Model) -> Self {
        Self { id: m.id, image_url: m.image_url, sort_order: m.sort_order }
    }
}

#[derive(Serialize, Default)]
pub struct RatingSummary {
    pub upvotes: i64,
    pub downvotes: i64,
    pub score: i64,
}

#[derive(Serialize)]
pub struct ReplyDto {
    pub id: u32,
    pub user_id: Uuid,
    pub body: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub author: UserBrief,
}

#[derive(Serialize)]
pub struct CommentDto {
    pub id: u32,
    pub user_id: Uuid,
    pub body: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub author: UserBrief,
    pub replies: Vec<ReplyDto>,
}

#[derive(Serialize)]
pub struct GigSummaryDto {
    pub id: u32,
    pub title: String,
    pub user_id: Uuid,
    pub category_id: u32,
    pub tags: Vec<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub author: UserBrief,
    pub primary_image: Option<String>,
    pub score: i64,
    pub comment_count: i64,
}

#[derive(Serialize)]
pub struct GigDetailDto {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub user_id: Uuid,
    pub category_id: u32,
    pub tags: Vec<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub author: UserBrief,
    pub images: Vec<ImageDto>,
    pub rating: RatingSummary,
    pub my_rating: Option<i32>,
    pub comments: Vec<CommentDto>,
    pub status: crate::entity::gig::GigStatus,
    pub budget_cents: Option<i64>,
    pub bid_count: i64,
    pub my_bid: Option<BidDto>,
    pub bids: Option<Vec<BidDto>>,
    pub my_order: Option<OrderSummaryDto>,
}

// ---------- helpers ----------

pub async fn fetch_users_map(
    db: &DatabaseConnection,
    ids: &[Uuid],
) -> Result<HashMap<Uuid, UserBrief>, AppError> {
    if ids.is_empty() {
        return Ok(HashMap::new());
    }
    let users = user::Entity::find()
        .filter(user::Column::Id.is_in(ids.iter().copied().collect::<Vec<_>>()))
        .all(db)
        .await?;
    Ok(users.iter().map(|u| (u.id, UserBrief::from(u))).collect())
}

pub async fn build_comment_tree(
    db: &DatabaseConnection,
    comments: Vec<gig_comment::Model>,
) -> Result<Vec<CommentDto>, AppError> {
    if comments.is_empty() {
        return Ok(Vec::new());
    }
    let comment_ids: Vec<u32> = comments.iter().map(|c| c.id).collect();
    let replies = gig_comment_reply::Entity::find()
        .filter(gig_comment_reply::Column::CommentId.is_in(comment_ids.clone()))
        .order_by_asc(gig_comment_reply::Column::CreatedAt)
        .all(db)
        .await?;

    let mut user_ids: Vec<Uuid> = comments.iter().map(|c| c.user_id).collect();
    user_ids.extend(replies.iter().map(|r| r.user_id));
    user_ids.sort();
    user_ids.dedup();
    let users = fetch_users_map(db, &user_ids).await?;

    let mut by_comment: HashMap<u32, Vec<gig_comment_reply::Model>> = HashMap::new();
    for r in replies {
        by_comment.entry(r.comment_id).or_default().push(r);
    }

    let out = comments
        .into_iter()
        .map(|c| {
            let author = users.get(&c.user_id).cloned().unwrap_or_else(|| UserBrief::unknown(c.user_id));
            let replies = by_comment.remove(&c.id).unwrap_or_default().into_iter().map(|r| {
                let rauthor = users.get(&r.user_id).cloned().unwrap_or_else(|| UserBrief::unknown(r.user_id));
                ReplyDto {
                    id: r.id, user_id: r.user_id, body: r.body,
                    created_at: r.created_at, updated_at: r.updated_at,
                    author: rauthor,
                }
            }).collect();
            CommentDto {
                id: c.id, user_id: c.user_id, body: c.body,
                created_at: c.created_at, updated_at: c.updated_at,
                author, replies,
            }
        })
        .collect();
    Ok(out)
}

// convenience: fetch one gig by id or 404
pub async fn require_gig(db: &DatabaseConnection, id: u32) -> Result<gig::Model, AppError> {
    gig::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("gig not found".into()))
}
