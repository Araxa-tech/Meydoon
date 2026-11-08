use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct TagList(pub Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum GigStatus {
    #[sea_orm(string_value = "open")]
    Open,
    #[sea_orm(string_value = "in_progress")]
    InProgress,
    #[sea_orm(string_value = "completed")]
    Completed,
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
}

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gigs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    pub title: String,
    pub user_id: Uuid,
    pub description: String,
    pub category_id: u32,
    pub status: GigStatus,
    pub budget_cents: Option<i64>,
    #[sea_orm(column_type = "Json")]
    pub tags: TagList,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<super::user::Entity>,

    #[sea_orm(belongs_to, from = "category_id", to = "id")]
    pub category: BelongsTo<super::gig_category::Entity>,

    #[sea_orm(has_many)]
    pub images: HasMany<super::gig_image::Entity>,

    #[sea_orm(has_many)]
    pub ratings: HasMany<super::gig_rating::Entity>,

    #[sea_orm(has_many)]
    pub comments: HasMany<super::gig_comment::Entity>,

    #[sea_orm(has_many)]
    pub bids: HasMany<super::gig_bid::Entity>,

    #[sea_orm(has_many)]
    pub orders: HasMany<super::order::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
