use async_trait::async_trait;
use domain::{Book, BookId, BookRepository};
use security::{Role, SECURITY_CONTEXT};
use serde::{Deserialize, Serialize};
use stano_starter_service::{component, service, ServiceError};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: String,
    pub title: String,
    pub author: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
}

#[component]
#[async_trait]
pub trait BookService: Send + Sync {
    async fn list_books(&self) -> Result<Vec<BookResponse>, ServiceError>;
    async fn get_book(&self, id: &str) -> Result<BookResponse, ServiceError>;
    async fn create_book(&self, req: CreateBookRequest) -> Result<BookResponse, ServiceError>;
    async fn delete_book(&self, id: &str) -> Result<(), ServiceError>;
}

#[service(dyn BookService)]
pub struct BookServiceImpl {
    book_repo: Arc<dyn BookRepository>,
}

#[async_trait]
impl BookService for BookServiceImpl {
    async fn list_books(&self) -> Result<Vec<BookResponse>, ServiceError> {
        let books = self.book_repo.find_all().await?;
        Ok(books
            .into_iter()
            .map(|b| BookResponse {
                id: b.id.to_string(),
                title: b.title,
                author: b.author,
            })
            .collect())
    }

    async fn get_book(&self, id: &str) -> Result<BookResponse, ServiceError> {
        use std::str::FromStr;
        let book_id = BookId::from_str(id)
            .map_err(|_| ServiceError::InvalidInput("Invalid book ID".into()))?;
        let book = self
            .book_repo
            .find_by_id(&book_id)
            .await?
            .ok_or(ServiceError::NotFound)?;
        Ok(BookResponse {
            id: book.id.to_string(),
            title: book.title,
            author: book.author,
        })
    }

    async fn create_book(&self, req: CreateBookRequest) -> Result<BookResponse, ServiceError> {
        SECURITY_CONTEXT
            .try_with(|_| ())
            .map_err(|_| ServiceError::Unauthorized)?;
        let book = Book::new(req.title, req.author);
        self.book_repo.create(&book).await?;
        Ok(BookResponse {
            id: book.id.to_string(),
            title: book.title,
            author: book.author,
        })
    }

    async fn delete_book(&self, id: &str) -> Result<(), ServiceError> {
        SECURITY_CONTEXT
            .try_with(|ctx| {
                if ctx.ext().role == Role::Admin {
                    Ok(())
                } else {
                    Err(ServiceError::Forbidden)
                }
            })
            .map_err(|_| ServiceError::Unauthorized)??;
        use std::str::FromStr;
        let book_id = BookId::from_str(id)
            .map_err(|_| ServiceError::InvalidInput("Invalid book ID".into()))?;
        self.book_repo.delete(&book_id).await?;
        Ok(())
    }
}
