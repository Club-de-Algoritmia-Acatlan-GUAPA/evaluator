use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

use crate::consts::CONFIGURATION;
pub async fn get_postgres_pool() -> PgPool {
    let config = &CONFIGURATION.postgres;
    let pg_uri = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.database
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_uri)
        .await
        .context("Failed postgres connection")
        .unwrap();

    info!("RUNNING postgres on {}", pg_uri);
    pool
}
