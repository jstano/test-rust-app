use anyhow::Result;
use sea_orm::DatabaseConnection;
use stano_seaorm::DbConfig;

pub async fn setup_db(url: &str) -> Result<DatabaseConnection> {
    migration::run_migrations(url).await?;
    let db = DbConfig::from_url(url).await?;
    Ok(db.connection().clone())
}
