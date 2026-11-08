use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(unique)]
    pub user_name: String,
    #[sea_orm(unique)]
    pub phone_number: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub is_verified: bool,
    pub birthday: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub verified_at: Option<DateTimeUtc>,
    pub premiumed_at: Option<DateTimeUtc>,

    #[sea_orm(has_one)]
    pub profile: HasOne<super::profile::Entity>,

    #[sea_orm(has_one)]
    #[serde(skip_serializing)]
    pub private_info: HasOne<super::user_private_information::Entity>,

    #[sea_orm(has_many)]
    #[serde(skip_serializing)]
    pub password_reset_tokens: HasMany<super::password_reset_token::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
