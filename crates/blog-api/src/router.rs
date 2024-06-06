use std::time::Duration;

use anyhow::Context;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::{routing::get, BoxError, Json, Router};
use lazy_static::lazy_static;
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] =
        &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct ApplicationCnotroller {}

impl ApplicationCnotroller {
    pub async fn serve(port: u32, cors_origin: &str) -> anyhow::Result<()> {
        let router = Router::new()
            .route("/api/ping", get(Self::ping))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(Self::handle_timeout_error))
                    .timeout(Duration::from_secs(*HTTP_TIMEOUT)),
            )
            .layer(
                CorsLayer::new()
                    .allow_origin(cors_origin.parse::<http::HeaderValue>()?)
                    .allow_methods([]),
            );

        info!("routes initialized");
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
            .await
            .context("error while binding port")?;
        info!("listening on port {}", port);
        axum::serve(listener, router)
            .await
            .context("error while starting the api server")?;

        Ok(())
    }

    async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
        if err.is::<tower::timeout::error::Elapsed>() {
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error":
                        format!(
                            "request took longer than the configured {} second timeout",
                            *HTTP_TIMEOUT
                        )
                })),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("unhandled internal error: {}", err) })),
            )
        }
    }

    async fn ping() -> &'static str {
        "pong"
    }
}
