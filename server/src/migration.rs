use sea_orm::{DatabaseConnection};


pub async fn migrate_db(db: DatabaseConnection) -> anyhow::Result<DatabaseConnection> {
    db.get_schema_registry("meydoon::entity::*")
        .sync(&db)
        .await?;

    // db.execute(sea_orm::Statement::from_string(
    //         sea_orm::DatabaseBackend::Sqlite,
    //         "CREATE UNIQUE INDEX IF NOT EXISTS ux_gig_bids_bidder_gig \
    //         ON gig_bids(bidder_id, gig_id)".to_string(),
    // )).await?;
    Ok(db)
}
