use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_sessions::extractors::WritableSession;

pub async fn logout(mut session: WritableSession) -> Response {
    session.destroy();
    StatusCode::OK.into_response()
}
