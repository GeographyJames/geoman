use std::time::Instant;

use db_migrate::{
    config,
    migrate::{commit, truncate_table, update_serial},
    tables::{self},
};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    let dry_run = false;
    println!("Starting migration");
    let (source_db, target_db) = config::get_db_connections()?;
    let schema = "app";

    let table = "teams";
    println!("Migrating {table}...");
    let start = Instant::now();
    let tables = tables::Team::select_all(&source_db).await?;
    let mut tx = target_db.begin().await.unwrap();
    truncate_table(schema, table, &mut *tx).await?;
    for table in tables {
        table.migrate(&mut *tx).await?;
    }
    update_serial(schema, table, &mut *tx).await?;
    commit(tx, dry_run).await?;
    println!("\x1b[32m✓ Migrated {table} in {:?}\x1b[0m", start.elapsed());

    let table = "users";
    println!("Migrating {table}...");
    let start = Instant::now();
    let tables = tables::User::select_all(&source_db).await?;
    let mut tx = target_db.begin().await.unwrap();
    truncate_table(schema, table, &mut *tx).await?;
    for table in tables {
        table.migrate(&mut *tx).await?;
    }
    update_serial(schema, table, &mut *tx).await?;
    commit(tx, dry_run).await?;
    println!("\x1b[32m✓ Migrated {table} in {:?}\x1b[0m", start.elapsed());

    println!("Creating root user");
    sqlx::query!("INSERT INTO app.users (username, first_name, last_name, admin) VALUES ('root-user', 'root', 'user', true)")
        .execute(&target_db)
        .await
        .expect("failed to insert root user");
    println!("\x1b[32m✓ Root user created\x1b[0m");

    let table = "search_areas";
    println!("Migrating {table}...");
    let start = Instant::now();
    let tables = tables::SearchArea::select_all(&source_db).await?;
    let mut tx = target_db.begin().await.unwrap();
    truncate_table(schema, table, &mut *tx).await?;
    for table in tables {
        table.migrate(&mut *tx).await?;
    }
    update_serial(schema, table, &mut *tx).await?;
    commit(tx, dry_run).await?;
    println!("\x1b[32m✓ Migrated {table} in {:?}\x1b[0m", start.elapsed());

    let table = "projects";
    println!("Migrating {table}...");
    let start = Instant::now();
    let tables = tables::Project::select_all(&source_db).await?;
    let mut tx = target_db.begin().await.unwrap();
    truncate_table(schema, table, &mut *tx).await?;
    for table in tables {
        table.migrate(&mut *tx).await?;
    }
    update_serial(schema, table, &mut *tx).await?;
    commit(tx, dry_run).await?;
    println!("\x1b[32m✓ Migrated {table} in {:?}\x1b[0m", start.elapsed());
    Ok(())
}
