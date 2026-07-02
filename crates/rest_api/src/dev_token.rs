use axum::{
    extract::{Query, State}, response::IntoResponse,
    routing::get,
    Json,
    Router,
};
use security::{AppClaims, Role};
use serde::{Deserialize, Serialize};
use stano_di::application_context::ApplicationContext;
use stano_launcher::BootstrapConfig;
use stano_security::encode_jwt;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    email: String,
    role: Option<String>,
}

#[derive(Debug, Serialize)]
struct TokenResponse {
    token: String,
}

pub fn dev_token_routes() -> Router<Arc<ApplicationContext>> {
    Router::new().route("/auth/dev-token", get(dev_token_handler))
}

async fn dev_token_handler(
    State(ctx): State<Arc<ApplicationContext>>,
    Query(req): Query<TokenRequest>,
) -> impl IntoResponse {
    let config = ctx.get::<BootstrapConfig>();
    let role = match req.role.as_deref() {
        Some("admin") => Role::Admin,
        _ => Role::User,
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = stano_security::Claims {
        sub: format!("user-{}", uuid::Uuid::new_v4()),
        session_id: format!("sess-{}", uuid::Uuid::new_v4()),
        exp: now + config.jwt_config.expiration_seconds as usize,
        ext: AppClaims {
            email: req.email,
            role,
        },
    };

    match encode_jwt(&claims, &config.jwt_config) {
        Ok(token) => Json(TokenResponse { token }).into_response(),
        Err(e) => {
            let response = stano_axum::ErrorResponse::new(
                500,
                "TOKEN_GENERATION_FAILED",
                format!("Failed to generate token: {}", e),
            );
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(response),
            )
                .into_response()
        }
    }
}
