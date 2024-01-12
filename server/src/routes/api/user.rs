use axum::body::{Body};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_sessions::extractors::ReadableSession;
use serde::{Deserialize, Serialize};

use crate::{AppError, AppState, UserModel};
use crate::routes::api::auth::login::LoginCredentials;

pub async fn get_users(
    State(state): State<AppState>,
    session: ReadableSession,
) -> Result<Json<Vec<UserModel>>, AppError> {
    let _user = session
        .get::<String>("user_id")
        .ok_or(AppError::StateNotFound)?;

    let results = sqlx::query_as::<_, UserModel>("SELECT * FROM users")
        .fetch_all(&*state.db)
        .await
        .unwrap();

    Ok(Json(results))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUserRequest {
    id: u64,
}

pub async fn delete_user(
    State(state): State<AppState>,
    session: ReadableSession,
    Json(payload): Json<DeleteUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let _user = session
        .get::<String>("user_id")
        .ok_or(AppError::StateNotFound)?;

    if payload.id.to_string() == _user {
        return Err(AppError::NotAllowed {
            error: "Cannot delete own user".to_string(),
        });
    }

    sqlx::query("DELETE FROM users where id = $1")
        .bind(payload.id.to_string())
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap())
}

pub async fn add_user(
    State(state): State<AppState>,
    session: ReadableSession,
    Json(payload): Json<LoginCredentials>,
) -> Result<impl IntoResponse, AppError> {
    let _user = session
        .get::<String>("user_id")
        .ok_or(AppError::StateNotFound);

    let previous = sqlx::query_as::<_, UserModel>("SELECT * FROM users where username = $1")
        .bind(&payload.username)
        .fetch_optional(&*state.db)
        .await
        .unwrap();

    match previous {
        Some(_) => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_ACCEPTABLE)
                .body(Body::from(String::from("User already exists")))
                .unwrap())
        }
        None => {
            let hash = bcrypt::hash(&payload.password, 12).unwrap();
            sqlx::query("INSERT INTO users (email,username, password) VALUES (?, ?, ?)")
                .bind(payload.email)
                .bind(payload.username)
                .bind(hash)
                .execute(&*state.db)
                .await
                .map_err(|_| AppError::InternalError)?;
        }
    }

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::empty())
        .unwrap())
}
