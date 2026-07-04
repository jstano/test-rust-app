use axum::{
    extract::{Path, State}, http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json,
    Router,
};
use services::{BookService, CreateBookRequest};
use stano_starter_rest::{application_context::ApplicationContext, AppJson};
use std::sync::Arc;

pub fn public_routes() -> Router<Arc<ApplicationContext>> {
    Router::new()
        .route("/books", get(list_books))
        .route("/books/:id", get(get_book))
}

pub fn protected_routes() -> Router<Arc<ApplicationContext>> {
    Router::new()
        .route("/books", post(create_book))
        .route("/books/:id", delete(delete_book))
}

async fn list_books(State(ctx): State<Arc<ApplicationContext>>) -> impl IntoResponse {
    match ctx.get_trait::<dyn BookService>().list_books().await {
        Ok(books) => (StatusCode::OK, Json(books)).into_response(),
        Err(e) => {
            let api_error = stano_starter_rest::ApiError::from(e);
            api_error.into_response()
        }
    }
}

async fn get_book(
    State(ctx): State<Arc<ApplicationContext>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match ctx.get_trait::<dyn BookService>().get_book(&id).await {
        Ok(book) => (StatusCode::OK, Json(book)).into_response(),
        Err(e) => {
            let api_error = stano_starter_rest::ApiError::from(e);
            api_error.into_response()
        }
    }
}

async fn create_book(
    State(ctx): State<Arc<ApplicationContext>>,
    AppJson(req): AppJson<CreateBookRequest>,
) -> impl IntoResponse {
    match ctx.get_trait::<dyn BookService>().create_book(req).await {
        Ok(book) => (StatusCode::CREATED, Json(book)).into_response(),
        Err(e) => {
            let api_error = stano_starter_rest::ApiError::from(e);
            api_error.into_response()
        }
    }
}

async fn delete_book(
    State(ctx): State<Arc<ApplicationContext>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match ctx.get_trait::<dyn BookService>().delete_book(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            let api_error = stano_starter_rest::ApiError::from(e);
            api_error.into_response()
        }
    }
}
