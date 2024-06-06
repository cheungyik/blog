use std::sync::Arc;

use anyhow::Context;
use clap::Parser;

use blog_api::router::ApplicationCnotroller;
use blog_core::config::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("failed to load .env file")?;

    let config = Arc::new(AppConfig::parse());

    let timer = time::format_description::parse(
        "[year]-[month padding:zero]-[day padding:zero]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]",
    )?;
    let time_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(time_offset, timer);
    tracing_subscriber::fmt().with_timer(timer).init();

    let port = config.port;
    ApplicationCnotroller::serve(port)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
