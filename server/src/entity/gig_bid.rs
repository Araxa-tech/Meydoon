use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum BidStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "accepted")]
    Accepted,
    #[sea_orm(string_value = "rejected")]
    Rejected,
    #[sea_orm(string_value = "withdrawn")]
    Withdrawn,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gig_bids")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,

    #[sea_orm(indexed)]
    pub gig_id: u32,
    #[sea_orm(indexed)]
    pub bidder_id: Uuid,

    pub amount_cents: i64,
    pub delivery_days: u32,
    pub message: String,
    pub status: BidStatus,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    #[sea_orm(belongs_to, from = "gig_id", to = "id")]
    pub gig: BelongsTo<super::gig::Entity>,

    #[sea_orm(belongs_to, from = "bidder_id", to = "id")]
    pub bidder: BelongsTo<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
