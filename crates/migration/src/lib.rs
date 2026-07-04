use anyhow::Result;
use schema_installer::{DirectoryMigrationSource, Migrator, SchemaInstallerConfigBuilder};
use schema_sql_generator::common::generator_type::GeneratorType;
use std::path::PathBuf;

pub async fn run_migrations(database_url: &str) -> Result<()> {
    let config = SchemaInstallerConfigBuilder::new()
        .database_type(GeneratorType::Postgresql)
        .connection_string(database_url.to_string())
        .build()?;

    let migrations_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations"));
    let source = Box::new(DirectoryMigrationSource { path: migrations_dir });

    Migrator::migrate(&config, source).await?;
    Ok(())
}
