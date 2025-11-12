//! Application data repository
pub mod models;
pub mod postgres;
mod traits;
pub use postgres::PostgresRepo;
