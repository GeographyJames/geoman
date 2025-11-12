//! Application data repository
pub mod models;
mod postgres;
mod traits;
pub use postgres::PostgresRepo;
