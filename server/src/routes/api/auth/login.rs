use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::models::user::UserModel;
use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::AppState;

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

    Ok(AppSuccess::OK {
        data: Some(serde_json::to_string(&user).unwrap()),
    }
    .into_response())
}
