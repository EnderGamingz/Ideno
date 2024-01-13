use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub enum AppError {
    StateNotFound,
    InternalError,
    UserNotFound,
    NotLoggedIn,
    BadRequest { error: Option<String> },
    NotAllowed { error: String },
    DataConflict { error: String },
    NotFound { error: String },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code;
        let mut body = "".to_string();

        match self {
            Self::StateNotFound => {
                status_code = StatusCode::UNAUTHORIZED;
            }
            Self::UserNotFound => {
                status_code = StatusCode::NOT_FOUND;
                body = "User not found".to_string();
            }
            Self::InternalError => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                body = "Internal server error".to_string();
            }
            Self::NotLoggedIn => {
                status_code = StatusCode::UNAUTHORIZED;
                body = "Not logged in".to_string();
            }
            Self::BadRequest { error } => {
                status_code = StatusCode::BAD_REQUEST;
                body = error.unwrap_or("".to_string());
            }
            Self::NotAllowed { error } => {
                status_code = StatusCode::FORBIDDEN;
                body = error;
            }
            Self::DataConflict { error } => {
                status_code = StatusCode::CONFLICT;
                body = error;
            }
            Self::NotFound { error } => {
                status_code = StatusCode::NOT_FOUND;
                body = error;
            }
        }

        (status_code, body).into_response()
    }
}
