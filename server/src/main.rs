extern crate core;

use std::net::SocketAddr;

use axum::http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use dotenv::dotenv;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Pool, Sqlite, SqlitePool};
use tower_http::cors::CorsLayer;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

use crate::services::account_service::AccountService;
use crate::services::certification_service::CertificationService;
use crate::services::contact_information_service::ContactInformationService;
use crate::services::education_service::EducationService;
use crate::services::experience_service::ExperienceService;
use crate::services::profile_service::ProfileService;
use crate::services::user_service::UserService;

mod models;
mod response;
mod router;
mod routes;
mod services;

pub type IdenoPool = Pool<Sqlite>;
pub type IdenoDBResult = SqliteQueryResult;

#[derive(Clone)]
pub struct AppState {
    user_service: UserService,
    profile_service: ProfileService,
    account_service: AccountService,
    certification_service: CertificationService,
    contact_information_service: ContactInformationService,
    education_service: EducationService,
    experience_service: ExperienceService,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .pretty()
        .with_target(false)
        .with_level(true)
        .with_file(false)
        .with_line_number(false)
        .compact()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cors_origin = std::env::var("CORS_ORIGIN")
        .expect("CORS_ORIGIN must be set")
        .parse::<HeaderValue>()
        .unwrap();

    tracing::info!(name: "bootstrap", "CORS_ORIGIN: {}", cors_origin.to_str().unwrap());

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_origin(cors_origin)
        .allow_headers([CONTENT_TYPE, ACCESS_CONTROL_ALLOW_CREDENTIALS])
        .allow_credentials(true);

    let store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(30)));

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = SqlitePool::connect(&*db_url).await.unwrap();

    tracing::info!(name: "bootstrap", "Connected to database at {}", db_url);

    sqlx::migrate!().run(&db).await.unwrap();

    tracing::info!(name: "bootstrap", "Migrated database");

    tracing::info!(name: "bootstrap", "Starting server");

    let user_service = UserService::new(db.clone());
    let profile_service = ProfileService::new(db.clone());
    let account_service = AccountService::new(db.clone());
    let certification_service = CertificationService::new(db.clone());
    let contact_information_service = ContactInformationService::new(db.clone());
    let education_service = EducationService::new(db.clone());
    let experience_service = ExperienceService::new(db.clone());

    let state = AppState {
        user_service,
        profile_service,
        account_service,
        certification_service,
        contact_information_service,
        education_service,
        experience_service,
    };

    let router = router::router(cors, session_layer, state);

    let port = std::env::var("PORT").unwrap_or(3000.to_string());

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));
    let listener = tokio::net::TcpListener::bind(&socket_addr).await.unwrap();

    tracing::info!(name: "bootstrap", "Listening on {}", socket_addr);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}
