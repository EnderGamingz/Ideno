use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::{AppState, UserModel};
use crate::routes::api::auth::login::LoginCredentials;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<LoginCredentials>,
) -> Response {
    let user_results = sqlx::query_as::<_, UserModel>("SELECT id, username, password FROM users")
        .fetch_all(&*state.db)
        .await
        .unwrap();

    if user_results.len() > 0 {
        return StatusCode::NOT_ACCEPTABLE.into_response();
    }

    match bcrypt::hash(payload.password, 12) {
        Ok(hash) => {
            let result =
                sqlx::query("INSERT INTO users (email, username, password) VALUES (?, ?, ?)")
                    .bind(payload.email)
                    .bind(payload.username)
                    .bind(hash)
                    .execute(&*state.db)
                    .await
                    .unwrap();
            println!("Query result: {:?}", result);
            StatusCode::CREATED.into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    StatusCode::OK.into_response()
}
