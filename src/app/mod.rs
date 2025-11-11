//! The main Actix Web application

mod config;

pub mod enums;
mod helpers;
mod startup;
mod urls;
mod utoipa;
pub use config::{DatabaseSettings, Password, get_config};
pub use startup::Application;
pub use urls::URLS;
pub use utoipa::ApiDoc;
mod handlers;
mod routes;
mod state;
pub mod telemetry;
pub use state::AppState;
