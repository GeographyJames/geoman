//! The main Actix Web application

mod config;
mod constants;
pub mod enums;
mod helpers;
mod startup;
mod urls;
pub use config::{DatabaseSettings, Password, get_config};
pub use startup::Application;
pub use urls::URLS;
mod handlers;
mod routes;
mod state;
pub mod telemetry;
pub use state::AppState;
