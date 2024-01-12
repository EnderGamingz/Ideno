extern crate core;

use std::sync::Arc;

use axum::headers::HeaderValue;
use axum::http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, CONTENT_TYPE};
use axum::http::Method;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_sessions::async_session::MemoryStore;
use axum_sessions::SessionLayer;
use dotenv::dotenv;
use serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite, SqlitePool};
use tower_http::cors::CorsLayer;

mod router;
mod routes;
mod models;

#[derive(Serialize)]
enum AppError {
    StateNotFound,
    InternalError,
    NotAllowed { error: String },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code;
        let mut body = "".to_string();

        match self {
            Self::StateNotFound => {
                status_code = StatusCode::UNAUTHORIZED;
            }
            Self::InternalError => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
            }
            Self::NotAllowed { error } => {
                status_code = StatusCode::FORBIDDEN;
                body = error;
            }
        }

        (status_code, body).into_response()
    }
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct LinksModel {
    id: i64,
    user_id: i64,
    source: String,
    destination: String,
}


#[derive(Clone)]
pub struct AppState {
    db: Arc<Pool<Sqlite>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_origin(
            std::env::var("CORS_ORIGIN")
                .expect("CORS_ORIGIN must be set")
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_headers([CONTENT_TYPE, ACCESS_CONTROL_ALLOW_CREDENTIALS])
        .allow_credentials(true);

    let store = MemoryStore::new();

    let secret = [0; 128];
    let session_layer = SessionLayer::new(store, &secret).with_secure(false);

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = SqlitePool::connect(&*db_url).await.unwrap();

    sqlx::migrate!().run(&db).await.unwrap();

    let db = Arc::new(db);

    let state = AppState { db };

    let router = router::router(cors, session_layer, state);

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap()
}
