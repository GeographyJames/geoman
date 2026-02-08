use crate::common::{
    Auth, configure_database,
    constants::REQUEST_FAILED,
    helpers::{generate_random_bng_point_ewkt, handle_json_response},
    services::{
        ApiKeysService, AuthService, ClerkAuthService, HttpClient, HttpService, OgcService,
    },
};
use app::{
    AppConfig, Application, AuthenticatedUser, DatabaseSettings, Password, URLS,
    constants::{GIS_DATA_SCHEMA, SITE_BOUNDARIES_COLLECTION_NAME},
    enums::GeoManEnvironment,
    get_config,
    handlers::{
        self,
        api::{
            app_settings::AppSettings, project_collections::CollectionReqPayload,
            projects::PostProjectPayload,
        },
    },
    telemetry::{get_subscriber, init_subscriber},
};
use domain::{
    FeatureId, ProjectCollectionId, ProjectFeatureId, ProjectId, TableName, TeamId, UserId,
    enums::GeometryType,
};
use dotenvy::dotenv;
use secrecy::ExposeSecret;
use serde::Serialize;
use serde_json::json;
use sqlx::{Connection, PgConnection, PgPool};
use std::{str::FromStr, sync::LazyLock};
use uuid::Uuid;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp<T: AuthService> {
    pub app_config: AppConfig,
    pub db_pool: PgPool,
    pub api_client: HttpClient,
    pub test_user_id: String,
    pub test_user_2_id: String,
    pub auth: T,
    pub health_check_service: HttpService,
    pub ogc_service: OgcService,
    pub api_keys_service: ApiKeysService,
    pub projects_service: HttpService,
    pub users_service: HttpService,
    pub collections_service: HttpService,
    pub features_service: HttpService,
}

pub struct AppBuilder {
    pub with_db: bool,

    pub environment: Option<GeoManEnvironment>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            with_db: true,
            environment: None,
        }
    }
    pub async fn build(self) -> TestApp<ClerkAuthService> {
        let app = TestApp::spawn(self.environment).await;
        if self.with_db {
            configure_database(&app.app_config.db_settings).await
        }

        app
    }
    pub fn set_env(mut self, env: GeoManEnvironment) -> Self {
        self.environment = Some(env);
        self
    }
}

impl TestApp<ClerkAuthService> {
    pub async fn spawn(run_env: Option<GeoManEnvironment>) -> Self {
        dotenv().ok();
        LazyLock::force(&TRACING);
        let db_name = Uuid::new_v4().to_string();
        let mut config = get_config().expect("failed to intialise app config");
        config.db_settings.database_name = db_name.clone();
        // Set port to 0 so TCP Listner binds to random free port for tests
        config.app_settings.port = 0;
        let db_settings = config.db_settings.clone();
        let key1 = "TEST_USER_ID";
        let key2 = "TEST_USER_ID_2";
        let test_user_id =
            std::env::var(key1).expect(&format!("no {key1} environment variable set"));
        let test_user_2_id =
            std::env::var(key2).expect(&format!("no {key2} environment variable set"));
        let auth = ClerkAuthService {
            secret: secrecy::SecretBox::new(Box::new(
                config
                    .auth_settings
                    .clerk_secret_key
                    .expose_secret()
                    .to_owned(),
            )),
        };

        // Set environment for running the app
        if let Some(env) = run_env {
            config.app_settings.environment.run = env
        } else if let Ok(env) = std::env::var("GEOMAN_TEST_ENVIRONMENT") {
            config.app_settings.environment.run =
                GeoManEnvironment::from_str(&env).expect("invlaid test environment");
        };

        tracing::info!(
            "Spawning GeoMan test app for run environment '{}'",
            config.app_settings.environment.run
        );
        let app = Application::build(config.clone())
            .await
            .expect("failed to build application");
        let api_client = HttpClient::new(format!("http://127.0.0.1:{}", app.port));
        let _ = tokio::spawn(app.run_untill_stopped());

        let db_pool = db_settings.get_connection_pool();

        Self {
            app_config: config,
            db_pool,
            api_client,
            test_user_id,
            test_user_2_id,
            auth,
            health_check_service: HttpService {
                endpoint: URLS.health_check.clone(),
            },
            ogc_service: OgcService {},
            api_keys_service: ApiKeysService {
                endpoint: format!("{}{}", URLS.api.base, URLS.api.keys),
            },
            projects_service: HttpService {
                endpoint: format!("{}{}", URLS.api.base, URLS.api.projects),
            },
            users_service: HttpService {
                endpoint: format!("{}{}", URLS.api.base, URLS.api.users),
            },
            collections_service: HttpService {
                endpoint: format!("{}{}", URLS.api.base, URLS.api.collections),
            },
            features_service: HttpService {
                endpoint: format!("{}{}", URLS.api.base, URLS.api.project_features),
            },
        }
    }

    pub async fn spawn_with_db() -> Self {
        let app = Self::spawn(None).await;
        configure_database(&app.app_config.db_settings).await;
        app
    }

    pub async fn generate_project_collection_id(&self, auth: Option<&Auth>) -> ProjectCollectionId {
        let collection = CollectionReqPayload::default();
        let response = self
            .collections_service
            .post_json(&self.api_client, auth, &collection)
            .await;
        handle_json_response(response)
            .await
            .expect("failed to retrieve collection id")
    }

    pub async fn insert_project_feature_with_id<P: Serialize>(
        &self,
        id: i32,
        project_id: ProjectId,
        collection_id: ProjectCollectionId,
        user_id: UserId,
        properties: Option<P>,
    ) -> ProjectFeatureId {
        let record = sqlx::query!(
            "WITH inserted_feature AS (
                INSERT INTO app.project_features (
                id,
                    project_id,
                    collection_id,
                    name,
                    added_by,
                    last_updated_by,
                    properties
                ) 
                VALUES ($1, $2, $3, $4, $5, $5, $6) RETURNING id
             )
            INSERT INTO app.feature_objects (
                project_feature_id,
                collection_id,
                geom
                )
            SELECT id, $3, ST_GeomFromEWKT('SRID=27700;POINT(1 1)')
            FROM inserted_feature
            RETURNING project_feature_id, collection_id",
            id,
            project_id.0,
            collection_id.0,
            uuid::Uuid::new_v4().to_string(),
            user_id.0,
            json!(properties)
        )
        .fetch_one(&self.db_pool)
        .await
        .expect("failed to save feature to database");
        ProjectFeatureId {
            collection_id: ProjectCollectionId(record.collection_id),
            feature_id: domain::FeatureId(record.project_feature_id),
        }
    }

    pub async fn insert_project_feature<P: Serialize>(
        &self,
        name: &str,
        collection_id: ProjectCollectionId,
        project_id: ProjectId,
        user_id: UserId,
        geom_ewkt: &str,
        properties: Option<P>,
    ) -> ProjectFeatureId {
        let record = sqlx::query!(
            "WITH inserted_feature AS (
                INSERT INTO app.project_features (
                    project_id,
                    collection_id,
                    name,
                    added_by,
                    last_updated_by,
                    properties
                ) 
                VALUES ($1, $2, $3, $5, $5, $6) RETURNING id
             )
            INSERT INTO app.feature_objects (
                project_feature_id,
                collection_id,
                geom
                )
            SELECT id, $2, ST_GeomFromEWKT($4)
            FROM inserted_feature
            RETURNING project_feature_id, collection_id",
            project_id.0,
            collection_id.0,
            name,
            geom_ewkt,
            user_id.0,
            json!(properties)
        )
        .fetch_one(&self.db_pool)
        .await
        .expect("Failed to save feature in database");
        ProjectFeatureId {
            collection_id: ProjectCollectionId(record.collection_id),
            feature_id: FeatureId(record.project_feature_id),
        }
    }

    pub async fn generate_project_feature_id<P: Serialize>(
        &self,
        collection_id: ProjectCollectionId,
        project_id: ProjectId,
        user_id: UserId,
        properties: Option<P>,
    ) -> ProjectFeatureId {
        let (_, _, geom_wkt) = generate_random_bng_point_ewkt();
        self.insert_project_feature(
            &uuid::Uuid::new_v4().to_string(),
            collection_id,
            project_id,
            user_id,
            &geom_wkt,
            properties,
        )
        .await
    }

    pub async fn drop_database(&self) {
        let superuser_settings = DatabaseSettings {
            database_name: "postgres".to_string(),
            username: "postgres".to_string(),
            password: Password::new("password".to_string()),
            ..self.app_config.db_settings.clone()
        };
        let mut superuser_connection =
            PgConnection::connect_with(&superuser_settings.connect_options())
                .await
                .expect("failed to connect to database as superuser");
        // Now drop the database
        sqlx::query(&format!(
            r#"DROP DATABASE "{}""#,
            self.app_config.db_settings.database_name
        ))
        .execute(&mut superuser_connection)
        .await
        .expect("failed to drop database");
    }

    pub async fn generate_ids(&self) -> (TeamId, UserId, ProjectId) {
        let team_id = TeamId(0);
        let user_id = UserId(0);
        let project_id = self
            .generate_project_id(Some(&Auth::mock_session_token()))
            .await;
        (team_id, user_id, project_id)
    }

    pub async fn create_gis_data_table(
        &self,
        table_name: &TableName,
        geometry: &GeometryType,
        srid: u16,
        description: Option<&String>,
    ) {
        sqlx::query(&format!(
            r#"
        CREATE TABLE {GIS_DATA_SCHEMA}."{}" (
            gid integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
            geom geometry({},{}) NOT NULL,
            some_text TEXT
            )"#,
            table_name, geometry, srid
        ))
        .execute(&self.db_pool)
        .await
        .expect("failed to insert table");
        if let Some(desc) = description {
            sqlx::query(&format!(
                r#"COMMENT ON TABLE {GIS_DATA_SCHEMA}."{}" IS '{desc}'"#,
                table_name
            ))
            .execute(&self.db_pool)
            .await
            .expect("failed to add description");
        }
    }
    pub async fn generate_gis_data_table_name(&self) -> TableName {
        let table_name = TableName::parse(format!("table_{}", uuid::Uuid::new_v4())).unwrap();
        self.create_gis_data_table(&table_name, &GeometryType::Point, 4326, None)
            .await;
        table_name
    }
    pub async fn insert_feature(&self, table_name: &TableName, ewkt_geom: &str, text: &str) -> i32 {
        sqlx::query_scalar(&format!(
            r#"
        INSERT INTO "{}"."{}" (geom, some_text) VALUES (ST_GeomFromEWKT($1), $2) RETURNING gid
        "#,
            GIS_DATA_SCHEMA,
            table_name.as_ref()
        ))
        .bind(ewkt_geom)
        .bind(text)
        .fetch_one(&self.db_pool)
        .await
        .expect("failed to insert feature")
    }

    pub async fn generate_primary_boundary_id(
        &self,
        project_id: ProjectId,
        user_id: UserId,
    ) -> ProjectFeatureId {
        let mut tx = self.db_pool.begin().await.unwrap();
        let collection_id = sqlx::query_scalar!(
            "SELECT id FROM app.collections c WHERE c.title = $1",
            SITE_BOUNDARIES_COLLECTION_NAME
        )
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        let feature_id = sqlx::query_scalar!(
            "INSERT INTO app.project_features (
                    project_id,
                    collection_id,
                    name,
                    added_by,
                    last_updated_by,
                    is_primary
            ) VALUES (
                     $1, $2, $3, $4, $4, true
                     )
            RETURNING id",
            project_id.0,
            collection_id,
            uuid::Uuid::new_v4().to_string(),
            user_id.0
        )
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        let _object_id = sqlx::query_scalar!(
            "INSERT INTO app.feature_objects (collection_id, project_feature_id, geom) VALUES ($1, $2, ST_GeomFromEwkt('SRID=27700;MULTIPOLYGON (((30 10, 40 40, 20 40, 10 20, 30 10)))')) RETURNING id",
            collection_id,
            feature_id
        ).fetch_one(&mut *tx).await.unwrap();
        tx.commit().await.unwrap();
        ProjectFeatureId {
            collection_id: ProjectCollectionId(collection_id),
            feature_id: domain::FeatureId(feature_id),
        }
    }

    pub async fn generate_api_key(
        &self,
        auth: Option<&Auth>,
    ) -> handlers::api::keys::ApiKeyResPayload {
        let response = self
            .api_keys_service
            .generate_api_key(&self.api_client, auth)
            .await;
        let key: handlers::api::keys::ApiKeyResPayload = handle_json_response(response)
            .await
            .expect("failed to retrieve key");
        key
    }

    pub async fn generate_project_id(&self, auth: Option<&Auth>) -> ProjectId {
        let project = PostProjectPayload::default();
        let response = self
            .projects_service
            .post_json(&self.api_client, auth, &project)
            .await;
        handle_json_response(response)
            .await
            .expect("failed to retrieve project id")
    }
    pub async fn get_app_settings(&self) -> AppSettings {
        handle_json_response(
            self.api_client
                .get(format!("{}{}", &URLS.api.base, &URLS.api.app_settings))
                .send()
                .await
                .expect(REQUEST_FAILED),
        )
        .await
        .expect("failed to retrieve app settings ")
    }

    pub async fn _generate_user(&self, admin: bool, team_id: TeamId) -> AuthenticatedUser {
        let first_name = uuid::Uuid::new_v4().to_string();
        let last_name = uuid::Uuid::new_v4().to_string();

        let user_id = sqlx::query_scalar!(
            "INSERT INTO app.users (first_name, last_name, admin, team_id) VALUES ($1, $2, $3, $4) RETURNING id",
first_name, last_name,
            admin,
            team_id.0
        ).fetch_one(&self.db_pool).await.unwrap();
        AuthenticatedUser {
            id: UserId(user_id),
            first_name,
            last_name,
            username: None,
            team_id,
            admin,
        }
    }
}
