#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env)]
    pub database_url: String,
    #[clap(long, env)]
    pub rust_log: String,
    #[clap(long, env)]
    pub port: u32,
    #[clap(long, env)]
    pub cors_origin: String,
}
