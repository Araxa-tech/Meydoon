use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct StringList(pub Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum OrderStatus {
    #[sea_orm(string_value = "active")]
    Active,
    #[sea_orm(string_value = "delivered")]
    Delivered,
    #[sea_orm(string_value = "completed")]
    Completed,
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
    #[sea_orm(string_value = "disputed")]
    Disputed,
}

impl OrderStatus {
    pub fn can_transition_to(self, next: OrderStatus) -> bool {
        use OrderStatus::*;
        matches!(
            (self, next),
            (Active, Delivered) | (Active, Cancelled) | (Active, Disputed)
                | (Delivered, Completed) | (Delivered, Disputed)
        )
    }
    pub fn is_terminal(self) -> bool {
        matches!(self, OrderStatus::Completed | OrderStatus::Cancelled)
    }
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,

    /// 1:1 with the bid that produced this order; unique so a bid can never
    /// be accepted twice. Nullable so direct (non-bid) orders are allowed.
    #[sea_orm(unique)]
    pub bid_id: Option<u32>,

    #[sea_orm(indexed)]
    pub gig_id: u32,

    #[sea_orm(indexed)]
    pub client_id: Uuid,
    #[sea_orm(indexed)]
    pub freelancer_id: Uuid,

    pub amount_cents: i64,
    pub delivery_days: u32,
    pub status: OrderStatus,

    pub delivery_message: Option<String>,
    #[sea_orm(column_type = "Json")]
    pub delivery_urls: Option<StringList>,

    pub cancelled_by: Option<Uuid>,
    pub cancel_reason: Option<String>,
    pub dispute_reason: Option<String>,

    pub delivered_at: Option<DateTimeUtc>,
    pub completed_at: Option<DateTimeUtc>,
    pub cancelled_at: Option<DateTimeUtc>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    #[sea_orm(belongs_to, from = "gig_id", to = "id")]
    pub gig: BelongsTo<super::gig::Entity>,

    #[sea_orm(belongs_to, from = "client_id", to = "id", relation_enum = "Client")]
    pub client: BelongsTo<super::user::Entity>,

    #[sea_orm(belongs_to, from = "freelancer_id", to = "id", relation_enum = "Freelancer")]
    pub freelancer: BelongsTo<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
