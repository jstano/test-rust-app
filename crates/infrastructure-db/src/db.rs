use anyhow::Result;
use migration::MigratorTrait;
use sea_orm::DatabaseConnection;
use stano_seaorm::DbConfig;

pub async fn setup_db(url: &str) -> Result<DatabaseConnection> {
    let db = DbConfig::from_url(url).await?;
    let connection = db.connection().clone();
    migration::Migrator::up(&connection, None).await?;
    Ok(connection)
}
