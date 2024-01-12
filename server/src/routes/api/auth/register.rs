use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::{AppError, AppState};
use crate::models::user::UserModel;

#[derive(Serialize, Deserialize)]
pub struct RegisterCredentials {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterCredentials>,
) -> Result<impl IntoResponse, AppError> {
    let user_results = sqlx::query_as::<_, UserModel>
        ("SELECT * FROM users WHERE username = ? OR email = ?")
        .bind(&payload.username)
        .bind(&payload.email)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    if Some(user_results).is_some() {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    }

    let password_hash = bcrypt::hash(payload.password, 12);

    match password_hash {
        Ok(hash) => {
            sqlx::query("INSERT INTO users (email, username, password) VALUES (?, ?, ?)")
                .bind(payload.email)
                .bind(payload.username)
                .bind(hash)
                .execute(&*state.db)
                .await
                .unwrap();
        }
        Err(_) => Err(AppError::InternalError)?,
    };

    Ok(StatusCode::CREATED.into_response())
}
