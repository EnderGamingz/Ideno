use std::collections::HashMap;

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_sessions::extractors::WritableSession;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::user::UserModel;
use crate::{AppError, AppState};

#[derive(Serialize, Deserialize)]
pub struct LoginCredentials {
    pub(crate) username: String,
    pub(crate) password: String,
}

pub async fn login(
    State(state): State<AppState>,
    mut session: WritableSession,
    Json(payload): Json<LoginCredentials>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(_user) = session.get::<String>("user_id") {
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

    session.insert("user_id", user.id.to_string()).unwrap();

    let user_without_password: HashMap<String, Value> =
        serde_json::from_str(&serde_json::to_string(&user).unwrap()).unwrap();
    let mut user_without_password_clone = user_without_password.clone();
    user_without_password_clone.remove("password");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(
            serde_json::to_string(&user_without_password_clone).unwrap(),
        ))
        .unwrap()
        .into_response())
}
