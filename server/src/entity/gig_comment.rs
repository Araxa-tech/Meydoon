use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gig_comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,

    pub gig_id: u32,
    pub user_id: Uuid,
    pub body: String,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    #[sea_orm(belongs_to, from = "gig_id", to = "id")]
    pub gig: BelongsTo<super::gig::Entity>,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<super::user::Entity>,

    #[sea_orm(has_many)]
    pub replies: HasMany<super::gig_comment_reply::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
