use axum::extract::{Path, State};
use axum::Json;
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::AppState;

pub async fn get_public_certifications(
    State(state): State<AppState>,
    session: Session,
    Path(identifier): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let optional_user = state.user_service.check_user_optional(&session).await?;
    let user = state.user_service.get_user_by_username(identifier).await?;

    let certifications = match optional_user {
        Some(logged_in_user) if logged_in_user.id == user.id => {
            let certifications = state
                .certification_service
                .get_authenticated_certifications(logged_in_user.id)
                .await?;
            serde_json::to_value(&certifications).unwrap()
        }
        _ => {
            let certifications = state
                .certification_service
                .get_public_certifications(user.id, None)
                .await?;
            serde_json::to_value(&certifications).unwrap()
        }
    };

    Ok(Json(certifications))
}
