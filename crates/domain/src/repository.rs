use crate::book::{Book, BookId};
use async_trait::async_trait;
use stano_starter_domain::{component, ServiceError};

#[component]
#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Book>, ServiceError>;
    async fn find_by_id(&self, id: &BookId) -> Result<Option<Book>, ServiceError>;
    async fn create(&self, book: &Book) -> Result<(), ServiceError>;
    async fn delete(&self, id: &BookId) -> Result<bool, ServiceError>;
}
