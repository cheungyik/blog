use anyhow::Context;
use axum::{routing::get, Router};
use tracing::info;

pub struct ApplicationCnotroller {}

impl ApplicationCnotroller {
    pub async fn serve(port: u32) -> anyhow::Result<()> {
        let app = Router::new().route("/api/ping", get(Self::ping));

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
            .await
            .context("error while binding port")?;
        info!("listening on port {}", port);

        axum::serve(listener, app)
            .await
            .context("error while starting the api server")?;

        Ok(())
    }

    async fn ping() -> &'static str {
        "pong"
    }
}
