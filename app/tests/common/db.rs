use app::{DatabaseSettings, Password};
use sqlx::{Connection, Executor, PgConnection, PgPool};

pub async fn configure_database(db_settings: &DatabaseSettings) {
    // Connect as app user to maintenance database
    let app_user_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        ..db_settings.clone()
    };

    let mut app_connection = PgConnection::connect_with(&app_user_settings.connect_options())
        .await
        .expect(&format!(
            "failed to connect to maintenance database as '{}'",
            db_settings.username
        ));

    // Create database
    app_connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_settings.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Connect as superuser to new database
    let superuser_settings = DatabaseSettings {
        database_name: db_settings.database_name.clone(),
        username: "postgres".to_string(),
        password: Password::new("password".to_string()), // password used in init_db.sh script
        ..db_settings.clone()
    };

    let mut superuser_connection =
        PgConnection::connect_with(&superuser_settings.connect_options())
            .await
            .expect("Failed to connect to database as superuser");

    // Create PostGIS extension on new database
    superuser_connection
        .execute("CREATE EXTENSION postgis")
        .await
        .expect("Failed to create PostGIS extension");

    superuser_connection
        .execute(
            format!(
                "GRANT REFERENCES ON spatial_ref_sys TO {};",
                app_user_settings.username
            )
            .as_str(),
        )
        .await
        .expect(&format!(
            "Failed to grant REFERENCES on spatial_ref_sys to {}",
            app_user_settings.username
        ));
    // Migrate database
    let connection_pool = PgPool::connect_with(db_settings.connect_options())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!(".././migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    // Seed data
    // let seed_data_directory = PathBuf::from("../seed_data");
    // let sql_files = [""];
    // for file in sql_files {
    //     let mut path = seed_data_directory.clone();
    //     path.push(file);
    //     let query_string = fs::read_to_string(&path).expect("failed to read SQL file");
    //     connection_pool
    //         .execute(query_string.as_str())
    //         .await
    //         .expect("failed to execute sql file");
}
