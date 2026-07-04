use async_trait::async_trait;
use domain::{Book, BookId, BookRepository};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use stano_seaorm::Mapper;
use stano_starter::{service, ServiceError};
use std::sync::Arc;

use super::entity as book;

#[service(dyn BookRepository)]
pub struct SeaOrmBookRepository {
    db: Arc<DatabaseConnection>,
}

#[async_trait]
impl BookRepository for SeaOrmBookRepository {
    async fn find_all(&self) -> Result<Vec<Book>, ServiceError> {
        book::Entity::find()
            .all(self.db.as_ref())
            .await
            .map(|models| models.into_iter().map(book::Model::to_domain).collect())
            .map_err(|e| ServiceError::Internal(anyhow::anyhow!(e)))
    }

    async fn find_by_id(&self, id: &BookId) -> Result<Option<Book>, ServiceError> {
        book::Entity::find_by_id(*id.as_uuid())
            .one(self.db.as_ref())
            .await
            .map(|m| m.map(book::Model::to_domain))
            .map_err(|e| ServiceError::Internal(anyhow::anyhow!(e)))
    }

    async fn create(&self, book: &Book) -> Result<(), ServiceError> {
        let active_model = <book::Model as Mapper<Book>>::to_active_model(book);
        book::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await
            .map(|_| ())
            .map_err(|e| ServiceError::Internal(anyhow::anyhow!(e)))
    }

    async fn delete(&self, id: &BookId) -> Result<bool, ServiceError> {
        let result = book::Entity::delete_by_id(*id.as_uuid())
            .exec(self.db.as_ref())
            .await
            .map_err(|e| ServiceError::Internal(anyhow::anyhow!(e)))?;
        Ok(result.rows_affected > 0)
    }
}

impl Mapper<Book> for book::Model {
    type Model = book::Model;
    type ActiveModel = book::ActiveModel;

    fn to_domain(model: Self::Model) -> Book {
        Book {
            id: BookId::from(model.id),
            title: model.title,
            author: model.author,
        }
    }

    fn to_active_model(book: &Book) -> Self::ActiveModel {
        book::ActiveModel {
            id: Set(*book.id.as_uuid()),
            title: Set(book.title.clone()),
            author: Set(book.author.clone()),
        }
    }
}
