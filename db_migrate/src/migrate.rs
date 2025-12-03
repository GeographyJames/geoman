use anyhow::Context;
use sqlx::{Postgres, Transaction};

pub async fn truncate_table<'a, E>(
    schema: &str,
    table: &str,
    executor: E,
) -> Result<(), anyhow::Error>
where
    E: sqlx::PgExecutor<'a>,
{
    sqlx::query(&format!(r#"TRUNCATE "{schema}"."{table}" CASCADE"#))
        .execute(executor)
        .await
        .context(format!("failed to truncate {table} table"))?;
    Ok(())
}

pub async fn update_serial<'a, E>(
    schema: &str,
    table: &str,
    executor: E,
) -> Result<(), anyhow::Error>
where
    E: sqlx::PgExecutor<'a>,
{
    sqlx::query(&format!(
        r#"SELECT setval((SELECT pg_get_serial_sequence('{schema}.{table}', 'id')), (SELECT MAX(id) FROM "{schema}"."{table}"))"#
    )).execute(executor).await.context(format!("failed to update {table} id serial"))?;
    Ok(())
}

pub async fn commit<'a>(tx: Transaction<'a, Postgres>, dry_run: bool) -> Result<(), anyhow::Error> {
    match dry_run {
        true => println!("Dry run: rolling back"),
        false => {
            tx.commit().await.context("Failed to commit migration")?;
            println!("Migration completed successfully");
        }
    }
    Ok(())
}
