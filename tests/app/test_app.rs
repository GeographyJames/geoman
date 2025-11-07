use crate::app::{
    auth::clerk::ClerkAuthProvider,
    configure_database,
    constants::CLERK_USER_ID_KEY,
    services::{HttpClient, HttpService, OgcService},
};
use dotenvy::dotenv;
use geoman::{
    app::{
        Application, DatabaseSettings, URLS,
        enums::GeoManEnvironment,
        get_config,
        telemetry::{get_subscriber, init_subscriber},
    },
    domain::{CollectionId, FeatureId, GeometryType, ProjectId, Slug, TeamId, UserId},
};
use rand::Rng;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::sync::LazyLock;
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

pub struct TestApp {
    db_settings: DatabaseSettings,
    pub db_pool: PgPool,
    pub api_client: HttpClient,
    pub auth: ClerkAuthProvider,
    pub health_check_service: HttpService,
    pub api_docs_service: HttpService,
    pub ogc_service: OgcService,
}

impl TestApp {
    pub async fn spawn() -> Self {
        dotenv().ok();
        LazyLock::force(&TRACING);
        let db_name = Uuid::new_v4().to_string();
        let mut config = get_config().expect("failed to intialise app config");
        config.db_settings.database_name = db_name.clone();
        // Set port to 0 so TCP Listner binds to random free port for tests
        config.app_settings.port = 0;
        let db_settings = config.db_settings.clone();
        let test_user_id = std::env::var(CLERK_USER_ID_KEY)
            .expect(&format!("no {CLERK_USER_ID_KEY} environment variable set"));
        let auth = ClerkAuthProvider {
            secret: secrecy::SecretBox::new(Box::new(
                config
                    .auth_settings
                    .clerk_secret_key
                    .expose_secret()
                    .to_owned(),
            )),
            test_user_id,
        };
        // Set environment for running the app
        let testing_environment = GeoManEnvironment::Production;
        config.app_settings.environment = testing_environment.clone();
        tracing::info!(
            "Spawning GeoMan test app for environment '{}'",
            testing_environment
        );
        let app = Application::build(config)
            .await
            .expect("failed to build application");
        let api_client = HttpClient::new(format!("http://127.0.0.1:{}", app.port));
        let _ = tokio::spawn(app.run_untill_stopped());

        let db_pool = db_settings.get_connection_pool();

        Self {
            db_settings,
            db_pool,
            api_client,
            auth,
            health_check_service: HttpService {
                endpoint: URLS.health_check.clone(),
            },
            api_docs_service: HttpService {
                endpoint: format!("{}{}", &URLS.docs.base, &URLS.docs.api),
            },
            ogc_service: OgcService {},
        }
    }

    pub async fn spawn_with_db() -> Self {
        let app = Self::spawn().await;
        configure_database(&app.db_settings).await;
        app
    }

    pub async fn get_test_session_token(&self) -> String {
        self.auth
            .get_test_session_token(&self.api_client.client)
            .await
    }

    pub async fn insert_team(&self, name: &str) -> i32 {
        let record = sqlx::query!(
            "INSERT INTO app.teams (name) VALUES ($1) RETURNING id",
            name
        )
        .fetch_one(&self.db_pool)
        .await
        .expect("Failled to save team in database");
        record.id
    }

    pub async fn generate_team_id(&self) -> TeamId {
        TeamId(self.insert_team(&uuid::Uuid::new_v4().to_string()).await)
    }

    pub async fn insert_user(&self, team_id: TeamId) -> i32 {
        let record = sqlx::query!(
            "INSERT INTO app.users (team_id) VALUES ($1) RETURNING id",
            team_id.0
        )
        .fetch_one(&self.db_pool)
        .await
        .expect("Failed to save user in database");
        record.id
    }

    pub async fn generate_user_id(&self, team_id: TeamId) -> UserId {
        UserId(self.insert_user(team_id).await)
    }

    pub async fn insert_collection(
        &self,
        title: &str,
        slug: &Slug,
        geometry_type: GeometryType,
        user_id: UserId,
    ) -> i32 {
        let record = sqlx::query!(
            "INSERT INTO app.collections (title, slug, geometry_type, added_by, last_updated_by) VALUES ($1, $2, $3, $4, $4) RETURNING id",
            title,
            slug as &Slug,
            geometry_type as GeometryType,
            user_id.0
        ).fetch_one(&self.db_pool).await.expect("Failed to save collection in database");
        record.id
    }

    pub async fn generate_collection_slug_and_id(&self, user_id: UserId) -> (Slug, CollectionId) {
        let title = uuid::Uuid::new_v4().to_string();
        let slug = Slug::parse(title.clone()).expect("Failed to create slug");
        let collection_id = CollectionId(
            self.insert_collection(&title, &slug, GeometryType::MultiPolygon, user_id)
                .await,
        );
        (slug, collection_id)
    }

    pub async fn insert_project(&self, name: &str, slug: &Slug, user_id: UserId) -> i32 {
        let record = sqlx::query!(
            "INSERT INTO app.projects (name, slug, owner, added_by, last_updated_by) VALUES ($1, $2, $3, $3, $3) RETURNING id",
            name,
            slug as &Slug,
            user_id.0
        ).fetch_one(&self.db_pool).await.expect("Failed to save project in database");
        record.id
    }

    pub async fn generate_project_id(&self, user_id: UserId) -> ProjectId {
        let name = uuid::Uuid::new_v4().to_string();
        let slug = Slug::parse(name.clone()).expect("failed po create slug");
        ProjectId(self.insert_project(&name, &slug, user_id).await)
    }

    pub async fn insert_feature(
        &self,
        name: &str,
        collection_id: CollectionId,
        project_id: ProjectId,
        user_id: UserId,
        geom_ewkt: &str,
    ) -> i32 {
        let record = sqlx::query!(
            "INSERT INTO app.features (
                project_id,
                collection_id,
                name,
                geom,
                added_by,
                last_updated_by
            ) VALUES ($1, $2, $3, ST_GeomFromEWKT($4), $5, $5) RETURNING id",
            project_id.0,
            collection_id.0,
            name,
            geom_ewkt,
            user_id.0
        )
        .fetch_one(&self.db_pool)
        .await
        .expect("Failed to save feature in database");
        record.id
    }

    pub async fn generate_feature_id(
        &self,
        collection_id: CollectionId,
        project_id: ProjectId,
        user_id: UserId,
    ) -> FeatureId {
        let name = uuid::Uuid::new_v4().to_string();
        let mut rng = rand::rng();
        let easting: u32 = rng.random_range(..700_000);
        let northing: u32 = rng.random_range(..1_300_000);
        let geom_wkt = format!("SRID=27700;POINT({} {})", easting, northing);
        FeatureId(
            self.insert_feature(&name, collection_id, project_id, user_id, &geom_wkt)
                .await,
        )
    }
}
