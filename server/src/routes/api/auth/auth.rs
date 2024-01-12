use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_sessions::extractors::ReadableSession;

use crate::AppState;
use crate::models::user::UserModel;

pub async fn auth(State(state): State<AppState>, session: ReadableSession) -> impl IntoResponse {
    if let Some(_user) = session.get::<String>("user_id") {
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = $1")
            .bind(_user)
            .fetch_one(&*state.db)
            .await
            .unwrap();
        return Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(serde_json::to_string(&result).unwrap()))
            .unwrap()
            .into_response();
    };
    StatusCode::OK.into_response()
}
