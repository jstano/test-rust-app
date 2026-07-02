use axum::middleware;
use stano_di::{
    application_context::ApplicationContext,
    environment::{Environment, OsEnvironment},
};
use stano_launcher::{run, BootstrapConfig, RouteGroups};
use stano_security::JwtConfig;
use std::sync::Arc;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let env: Arc<dyn Environment> = Arc::new(OsEnvironment::new());

    let db_url = env.get("DATABASE_URL").expect("DATABASE_URL required");
    let db = infrastructure_db::setup_db(&db_url).await?;

    let config = BootstrapConfig {
        port: env.get("PORT").and_then(|p| p.parse().ok()).unwrap_or(3000),
        jwt_config: JwtConfig {
            private_key_pem: env
                .get("JWT_PRIVATE_KEY")
                .expect("JWT_PRIVATE_KEY required"),
            public_key_pem: env.get("JWT_PUBLIC_KEY").expect("JWT_PUBLIC_KEY required"),
            expiration_seconds: env
                .get("JWT_EXPIRATION_SECONDS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600),
        },
        cors_origins: vec![],
    };

    let mut ctx = ApplicationContext::new(env);
    ctx.register_instance(Arc::new(config.clone()));
    ctx.register_instance(Arc::new(db));
    ctx.register_all();
    ctx.validate().map_err(|errors| {
        anyhow::anyhow!(
            "DI container validation failed:\n{}",
            errors
                .iter()
                .map(|e| format!("  - {}", e))
                .collect::<Vec<_>>()
                .join("\n")
        )
    })?;

    let arc_ctx = Arc::new(ctx);

    let ctx_for_middleware = Arc::clone(&arc_ctx);
    let auth = middleware::from_fn(move |req, next| {
        let ctx = Arc::clone(&ctx_for_middleware);
        rest_api::auth_middleware(axum::extract::State(ctx), req, next)
    });

    let routes = RouteGroups {
        public: rest_api::public_routes().merge(rest_api::dev_token_routes()),
        protected: rest_api::protected_routes().layer(auth.clone()),
        admin: axum::Router::new().layer(auth),
    };

    run(arc_ctx, routes, config).await
}
