use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum RatingValue {
    #[sea_orm(num_value = 1)]
    Upvote,
    #[sea_orm(num_value = -1)]
    Downvote,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gig_ratings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,

    pub gig_id: u32,
    pub user_id: Uuid,

    pub value: RatingValue,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    #[sea_orm(belongs_to, from = "gig_id", to = "id")]
    pub gig: BelongsTo<super::gig::Entity>,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
