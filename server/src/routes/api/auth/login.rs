use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_sessions::extractors::WritableSession;
use serde::{Deserialize, Serialize};

use crate::{AppState, UserModel};

#[derive(Serialize, Deserialize)]
pub struct LoginCredentials {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
}
pub async fn login(
    State(state): State<AppState>,
    mut session: WritableSession,
    Json(payload): Json<LoginCredentials>,
) -> impl IntoResponse {
    if let Some(_user) = session.get::<String>("user_id") {
        return StatusCode::OK.into_response();
    }

    let result = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE username = $1")
        .bind(payload.username)
        .fetch_optional(&*state.db)
        .await
        .unwrap();

    let user = match result {
        Some(user) => user,
        None => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let password_flag = bcrypt::verify(payload.password, &*user.password).unwrap();

    if !password_flag {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    session.insert("user_id", user.id.to_string()).unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&user).unwrap()))
        .unwrap()
        .into_response()
}
