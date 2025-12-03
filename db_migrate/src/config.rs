use anyhow::Context;
use sqlx::{
    Pool, Postgres,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::str::FromStr;

pub fn get_db_connections() -> Result<(Pool<Postgres>, Pool<Postgres>), anyhow::Error> {
    // Load root .env first (for DATABASE_URL)
    dotenvy::from_filename(".env").ok();
    // Then load db_migrate specific .env (for SOURCE_DB_URL)
    dotenvy::from_filename("db_migrate/.env")
        .context("Failed to load .env file from db_migrate/.env")?;
    let source_db_conn = PgConnectOptions::from_str(
        &std::env::var("SOURCE_DB_URL").context("SOURCE_DB_URL not set")?,
    )
    .context("failed to create source db connection")?;

    let target_db_conn =
        PgConnectOptions::from_str(&std::env::var("DATABASE_URL").context("DATABASE_URL not set")?)
            .context("failed to connection to target database")?;

    let source_db = PgPoolOptions::new().connect_lazy_with(source_db_conn);
    let target_db = PgPoolOptions::new().connect_lazy_with(target_db_conn);
    Ok((source_db, target_db))
}
