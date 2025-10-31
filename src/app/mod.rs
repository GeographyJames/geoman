//! The main Actix Web application.

mod config;
mod constants;
mod enums;
mod helpers;
mod startup;
mod urls;
pub use config::get_config;
pub use startup::run;
pub use urls::URLS;
mod handlers;
