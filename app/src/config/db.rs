use secrecy::ExposeSecret;
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};

use crate::types::Password;

/// PostgreSQL database settings
#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Password,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username(&self.username)
            .password(self.password.expose_secret())
            .host(&self.host)
            .port(self.port)
            .database(&self.database_name)
            .application_name("GeoMan")
            .log_statements(log::LevelFilter::Trace)
    }

    pub fn get_connection_pool(&self) -> PgPool {
        PgPoolOptions::new().connect_lazy_with(self.connect_options())
    }
}
