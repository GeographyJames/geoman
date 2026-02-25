use crate::common::{
    Auth, configure_database,
    helpers::{
        add_layer, add_shapefile_to_form, create_gdal_multipolygon_bng, create_gdal_point_bng,
        create_shapefile_dataset, dataset_to_shapefile_data, handle_json_response,
    },
    services::{
        ApiKeysService, AuthService, ClerkAuthService, HttpClient, HttpService, OgcService,
    },
};
use app::{
    AppConfig, Application, AuthenticatedUser, DatabaseSettings, Password, URLS,
    constants::{GIS_DATA_SCHEMA, SITE_BOUNDARIES_COLLECTION_ID},
    enums::GeoManEnvironment,
    get_config,
    handlers::{
        self,
        api::{
            business_units::BusinessUnitInputPayload, project_collections::CollectionReqPayload,
            projects::PostProjectPayload, teams::TeamInputPayload,
        },
    },
    telemetry::{get_subscriber, init_subscriber},
};
use domain::{
    BusinessUnitId, FeatureId, ProjectCollectionId, ProjectFeatureId, ProjectId, TableName, TeamId,
    UserId, enums::GeometryType,
};
use dotenvy::dotenv;
use gdal::vector::{Geometry, LayerAccess};
use secrecy::ExposeSecret;

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
    pub epsg_service: HttpService,
    pub teams_service: HttpService,
    pub business_units_service: HttpService,
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
            epsg_service: HttpService {
                endpoint: format!("{}{}", URLS.api.base, URLS.api.epsg),
            },
            teams_service: HttpService {
                endpoint: format!("{}{}", URLS.api.base, URLS.api.teams),
            },
            business_units_service: HttpService {
                endpoint: format!("{}{}", URLS.api.base, URLS.api.business_units),
            },
        }
    }

    pub async fn spawn_with_db() -> Self {
        let app = Self::spawn(None).await;
        configure_database(&app.app_config.db_settings).await;
        app
    }

    pub async fn generate_project_collection_id(&self) -> ProjectCollectionId {
        let admin = self.generate_user(true, TeamId(0)).await;
        let admin_auth = Auth::MockUserCredentials(admin);
        let collection = CollectionReqPayload::default();
        let response = self
            .collections_service
            .post_json(&self.api_client, Some(&admin_auth), &collection)
            .await;
        handle_json_response(response)
            .await
            .expect("failed to retrieve collection id")
    }

    pub async fn insert_project_feature(
        &self,
        collection_id: ProjectCollectionId,
        project_id: ProjectId,
        geom: Geometry,
        srid: u32,
        auth: Option<&Auth>,
        is_primary: Option<bool>,
    ) -> ProjectFeatureId {
        let mut form = reqwest::multipart::Form::new();
        let (mut dataset, filename) = create_shapefile_dataset();
        let mut layer = add_layer(&mut dataset, geom.geometry_type(), srid);
        layer
            .create_feature(geom.clone())
            .expect("failed to add geom");
        let shapefile_data = dataset_to_shapefile_data(dataset, &filename);
        form = add_shapefile_to_form("test", shapefile_data, form)
            .text("name", uuid::Uuid::new_v4().to_string());
        let form = if let Some(is_primary) = is_primary {
            form.text("primary", is_primary.to_string())
        } else {
            form
        };
        let response = self
            .features_service
            .post_form(
                &self.api_client,
                form,
                format!("{}/{}", project_id, collection_id,),
                auth,
            )
            .await;
        let feature_id: FeatureId = handle_json_response(response).await.unwrap();
        ProjectFeatureId {
            collection_id,
            feature_id,
        }
    }

    pub async fn generate_project_feature_id(
        &self,
        collection_id: ProjectCollectionId,
        project_id: ProjectId,
        auth: Option<&Auth>,
    ) -> ProjectFeatureId {
        let geom = create_gdal_point_bng();
        self.insert_project_feature(collection_id, project_id, geom, 27700, auth, None)
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
        auth: Option<&Auth>,
    ) -> ProjectFeatureId {
        let polygon = create_gdal_multipolygon_bng();
        self.insert_project_feature(
            ProjectCollectionId(SITE_BOUNDARIES_COLLECTION_ID),
            project_id,
            polygon,
            27700,
            auth,
            Some(true),
        )
        .await
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
    pub async fn generate_bu_id(&self, auth: Option<&Auth>) -> BusinessUnitId {
        handle_json_response(
            self.business_units_service
                .post_json(
                    &self.api_client,
                    auth,
                    &BusinessUnitInputPayload {
                        name: uuid::Uuid::new_v4().to_string(),
                    },
                )
                .await,
        )
        .await
        .expect("failed to generate bu id")
    }

    pub async fn generate_team_id(&self, auth: Option<&Auth>) -> TeamId {
        handle_json_response(
            self.teams_service
                .post_json(
                    &self.api_client,
                    auth,
                    &TeamInputPayload {
                        name: uuid::Uuid::new_v4().to_string(),
                        business_unit: None,
                    },
                )
                .await,
        )
        .await
        .expect("failed to generate team id")
    }

    pub async fn generate_user(&self, admin: bool, team_id: TeamId) -> AuthenticatedUser {
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
