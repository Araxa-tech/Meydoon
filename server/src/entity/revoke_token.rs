use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "revoked_tokens")]
pub struct Model {
    /// The JWT `jti` claim being revoked. Using it as the PK also makes
    /// repeated logouts idempotent.
    #[sea_orm(primary_key, auto_increment = false)]
    pub jti: Uuid,

    /// When the underlying JWT expires. Rows can be pruned after this time.
    #[sea_orm(indexed)]
    pub expires_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}
