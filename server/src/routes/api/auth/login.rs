use std::collections::HashMap;

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tower_sessions::Session;

use crate::models::user::UserModel;
use crate::{AppError, AppState};

pub fn sanitize_user(user: UserModel) -> HashMap<String, Value> {
    let user_map: HashMap<String, Value> =
        serde_json::from_str(&serde_json::to_string(&user).unwrap()).unwrap();
    let mut sanitized_user = user_map.clone();
    sanitized_user.remove("password");

    sanitized_user
}

#[derive(Serialize, Deserialize)]
pub struct LoginCredentials {
    pub(crate) username: String,
    pub(crate) password: String,
}

pub async fn login(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<LoginCredentials>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(_user) = session.get::<String>("user_id").await.unwrap() {
        return Ok(StatusCode::OK.into_response());
    }

    let result =
        sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE username = $1 OR email = $1")
            .bind(payload.username)
            .fetch_optional(&*state.db)
            .await
            .map_err(|_| AppError::InternalError)?;

    let user = match result {
        Some(user) => user,
        None => {
            return Ok(StatusCode::UNAUTHORIZED.into_response());
        }
    };

    let is_password_valid =
        bcrypt::verify(payload.password, &*user.password).map_err(|_| AppError::InternalError)?;

    if !is_password_valid {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }

    session
        .insert("user_id", user.id.to_string())
        .await
        .unwrap();

    let sanitized_user = sanitize_user(user.clone());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&sanitized_user).unwrap()))
        .unwrap()
        .into_response())
}
