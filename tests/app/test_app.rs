use crate::app::{
    auth::clerk::ClerkAuthProvider,
    constants::CLERK_USER_ID_KEY,
    services::{HttpClient, HttpService, OgcService},
};
use dotenvy::dotenv;
use geoman::app::{
    Application, URLS, get_config,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use std::sync::LazyLock;

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
        let mut config = get_config().expect("failed to intialise app config");
        config.app_settings.environment = geoman::app::enums::GeoManEnvironment::Production;

        // Set port to 0 so TCP Listner binds to random free port for tests
        config.app_settings.port = 0;

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
        let app = Application::build(config)
            .await
            .expect("failed to build application");
        let api_client = HttpClient::new(format!("http://127.0.0.1:{}", app.port));
        let _ = tokio::spawn(app.run_untill_stopped());

        Self {
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
    pub async fn get_test_session_token(&self) -> String {
        self.auth
            .get_test_session_token(&self.api_client.client)
            .await
    }
}
