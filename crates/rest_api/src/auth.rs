use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use security::{AppSecurityContext, SECURITY_CONTEXT};
use stano_starter_rest::{application_context::ApplicationContext, decode_jwt, BootstrapConfig, ErrorResponse};
use std::sync::Arc;

pub async fn auth_middleware(
    State(ctx): State<Arc<ApplicationContext>>,
    request: Request,
    next: Next,
) -> Response {
    let config = ctx.get::<BootstrapConfig>();

    match extract_token(&request) {
        None => unauthorized("Missing credentials"),
        Some(token) => match decode_jwt::<security::AppClaims>(&token, &config.jwt_config) {
            Err(e) => {
                tracing::debug!(error = %e, "JWT validation failed");
                unauthorized("Invalid or expired token")
            }
            Ok(claims) => {
                let sc = AppSecurityContext::new(claims);
                SECURITY_CONTEXT.scope(sc, next.run(request)).await
            }
        },
    }
}

fn extract_token(request: &Request) -> Option<String> {
    if let Some(cookie_hdr) = request
        .headers()
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
    {
        for part in cookie_hdr.split(';') {
            let mut kv = part.splitn(2, '=');
            #[allow(clippy::collapsible_if)]
            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                if k.trim() == "jwt_token" {
                    return Some(v.trim().to_string());
                }
            }
        }
    }

    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|t| t.trim().to_string())
}

fn unauthorized(msg: &'static str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new(401, "UNAUTHORIZED", msg)),
    )
        .into_response()
}
