extern crate core;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, CONTENT_TYPE};
use axum::http::StatusCode;
use axum::http::{HeaderValue, Method};
use axum::response::{IntoResponse, Response};
use dotenv::dotenv;
use serde::Serialize;
use sqlx::{Pool, Sqlite, SqlitePool};
use tower_http::cors::CorsLayer;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

mod auth;
mod models;
mod router;
mod routes;

#[derive(Serialize)]
enum AppError {
    StateNotFound,
    InternalError,
    UserNotFound,
    NotLoggedIn,
    BadRequest { error: Option<String> },
    NotAllowed { error: String },
    DataConflict { error: String },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code;
        let mut body = "".to_string();

        match self {
            Self::StateNotFound => {
                status_code = StatusCode::UNAUTHORIZED;
            }
            Self::UserNotFound => {
                status_code = StatusCode::NOT_FOUND;
                body = "User not found".to_string();
            }
            Self::InternalError => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                body = "Internal server error".to_string();
            }
            Self::NotLoggedIn => {
                status_code = StatusCode::UNAUTHORIZED;
                body = "Not logged in".to_string();
            }
            Self::BadRequest { error } => {
                status_code = StatusCode::BAD_REQUEST;
                body = error.unwrap_or("".to_string());
            }
            Self::NotAllowed { error } => {
                status_code = StatusCode::FORBIDDEN;
                body = error;
            }
            Self::DataConflict { error } => {
                status_code = StatusCode::CONFLICT;
                body = error;
            }
        }

        (status_code, body).into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    db: Arc<Pool<Sqlite>>,
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

    let db = Arc::new(db);

    let state = AppState { db };

    let router = router::router(cors, session_layer, state);

    let port = std::env::var("PORT").unwrap_or(3000.to_string());

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));
    let listener = tokio::net::TcpListener::bind(&socket_addr).await.unwrap();

    tracing::info!(name: "bootstrap", "Listening on {}", socket_addr);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}
