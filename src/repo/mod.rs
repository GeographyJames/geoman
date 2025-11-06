//! Application data repository.
mod postgres;
pub use postgres::PostgresRepo;
pub mod ogc;
mod traits;
