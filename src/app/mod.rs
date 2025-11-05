//! The main Actix Web application.

mod config;
mod constants;
pub mod enums;
mod helpers;
mod startup;
mod urls;
pub use config::get_config;
pub use startup::Application;
pub use urls::URLS;
mod handlers;
mod routes;
pub mod telemetry;
