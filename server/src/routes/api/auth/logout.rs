use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tower_sessions::Session;

pub async fn logout(session: Session) -> Response {
    let session_exists = session.get::<String>("user_id").await.unwrap();

    if session_exists.is_none() {
        return StatusCode::NOT_MODIFIED.into_response();
    }

    session.delete().await.unwrap();
    StatusCode::OK.into_response()
}
