use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::models::user::UserModel;
use crate::{AppError, AppState};

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
    let user_results = sqlx::query_as::<_, UserModel>(
        "SELECT * FROM users WHERE username = ? OR email = ? LIMIT 1",
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .fetch_optional(&*state.db)
    .await
    .map_err(|err| {
        println!("Error: {}", err);
        AppError::InternalError
    })?;

    if user_results.is_some() {
        return Err(AppError::BadRequest {
            error: Some("User already exists".to_string()),
        })?;
    }

    let password_hash = bcrypt::hash(payload.password, 12);

    match password_hash {
        Ok(hash) => {
            let user = sqlx::query_as::<_, UserModel>(
                "INSERT INTO users (email, username, password) VALUES (?, ?, ?) RETURNING *",
            )
            .bind(payload.email)
            .bind(payload.username)
            .bind(hash)
            .fetch_one(&*state.db)
            .await
            .unwrap();

            sqlx::query("INSERT INTO profiles (user_id) VALUES (?)")
                .bind(user.id)
                .execute(&*state.db)
                .await
                .unwrap();
        }
        Err(_) => Err(AppError::InternalError)?,
    };

    Ok(StatusCode::CREATED.into_response())
}
