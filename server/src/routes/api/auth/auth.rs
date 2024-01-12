use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tower_sessions::Session;

use crate::models::user::UserModel;
use crate::routes::api::auth::login::sanitize_user;
use crate::{AppError, AppState};

pub async fn auth(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    if let Some(user_id) = session.get::<String>("user_id").await.unwrap() {
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&*state.db)
            .await
            .map_err(|_| AppError::UserNotFound)?;

        let sanitized_user = sanitize_user(result);
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(serde_json::to_string(&sanitized_user).unwrap()))
            .unwrap()
            .into_response());
    };
    Ok(StatusCode::OK.into_response())
}
