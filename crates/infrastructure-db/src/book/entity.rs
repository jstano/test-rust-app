use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq, Eq)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    pub title: String,
    pub author: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
