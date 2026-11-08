use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gig_images")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub gig_id: u32,
    pub image_url: String,
    pub sort_order: i32,

    #[sea_orm(belongs_to, from = "gig_id", to = "id")]
    #[serde(skip_serializing)]
    pub gig: BelongsTo<super::gig::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
