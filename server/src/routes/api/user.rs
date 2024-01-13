use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUserRequest {
    id: i32,
}

pub async fn delete_user(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<DeleteUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let _user = session
        .get::<String>("user_id")
        .await
        .unwrap()
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
