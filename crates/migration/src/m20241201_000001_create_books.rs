use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Books::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Books::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Books::Title).string().not_null())
                    .col(ColumnDef::new(Books::Author).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Books::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Books {
    Table,
    Id,
    Title,
    Author,
}
