use axum::extract::State;
use axum::Json;
use tower_sessions::Session;

use crate::AppState;
use crate::models::user::AuthUserModel;
use crate::response::error_handling::AppError;
use crate::services::session_service::SessionService;

pub async fn auth(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Some(user_id) = SessionService::get_session_id(&session).await {
        let result = state.user_service.get_auth_user(user_id).await?;

        let response_version = match result.role == "admin" {
            true => {
                AuthUserModel {
                    id: result.id,
                    username: result.username,
                    email: result.email,
                    role: Some(result.role),
                    created_at: result.created_at,
                }
            },
            false => {
                AuthUserModel {
                    id: result.id,
                    username: result.username,
                    email: result.email,
                    created_at: result.created_at,
                    role: None,
                }
            }
        };

        return Ok(Json(serde_json::to_value(response_version).unwrap()));
    };
    Err(AppError::NotLoggedIn)
}
