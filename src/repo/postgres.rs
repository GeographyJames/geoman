use sqlx::PgPool;

pub struct PostgresRepo {
    pub db_pool: PgPool,
}
