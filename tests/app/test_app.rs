use crate::app::{
    auth::clerk::ClerkAuthProvider,
    constants::CLERK_USER_ID_KEY,
    services::{HttpClient, HttpService, OgcService},
};
use dotenvy::dotenv;
use geoman::app::{
    URLS, get_config, run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use std::{net::TcpListener, sync::LazyLock};

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
    pub projects_service: HttpService,
    pub api_docs_service: HttpService,
    pub ogc_service: OgcService,
}

impl TestApp {
    pub async fn spawn() -> Self {
        dotenv().ok();
        LazyLock::force(&TRACING);
        let mut config = get_config().expect("failed to intialise app config");
        config.app_settings.environment = geoman::app::enums::GeoManEnvironment::Production;
        let listener = TcpListener::bind(format!("{}:0", config.app_settings.host))
            .expect("failed to bind to port");
        let port = listener.local_addr().unwrap().port();
        config.app_settings.port = port;
        let test_user_id = std::env::var(CLERK_USER_ID_KEY)
            .expect(&format!("no {CLERK_USER_ID_KEY} environment variable set"));
        let api_client = HttpClient::new(format!(
            "http://{}:{}",
            config.app_settings.host, config.app_settings.port
        ));
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
        let server = run(listener, config).expect("failed to run server");
        let _ = tokio::spawn(server);

        Self {
            api_client,
            auth,
            health_check_service: HttpService {
                endpoint: URLS.health_check.clone(),
            },
            projects_service: HttpService {
                endpoint: format!("{}{}", &URLS.api.base, &URLS.api.projects),
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
