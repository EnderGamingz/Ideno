use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub enum AppSuccess {
    OK { data: Option<String> },
    CREATED { id: Option<i64> },
    DELETED,
    UPDATED,
}

impl IntoResponse for AppSuccess {
    fn into_response(self) -> Response {
        let status_code;
        let mut body = "".to_string();

        match self {
            Self::OK { data } => {
                status_code = StatusCode::OK;
                if let Some(data) = data {
                    body = data;
                }
            }
            Self::CREATED { id } => {
                status_code = StatusCode::CREATED;

                if let Some(id) = id {
                    body = id.to_string();
                }
            }
            Self::DELETED => status_code = StatusCode::ACCEPTED,
            Self::UPDATED => status_code = StatusCode::ACCEPTED,
        }

        (status_code, body).into_response()
    }
}
