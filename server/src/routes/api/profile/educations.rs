use axum::extract::{Path, State};
use axum::Json;
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::AppState;

pub async fn get_public_educations(
    State(state): State<AppState>,
    session: Session,
    Path(identifier): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let optional_user = state.user_service.check_user_optional(&session).await?;
    let user = state.user_service.get_user_by_username(identifier).await?;

    let educations = match optional_user {
        Some(logged_in_user) if logged_in_user.id == user.id => {
            let educations = state
                .education_service
                .get_authenticated_educations(logged_in_user.id)
                .await?;
            serde_json::to_value(&educations).unwrap()
        }
        _ => {
            let educations = state
                .education_service
                .get_public_educations(user.id, None)
                .await?;
            serde_json::to_value(&educations).unwrap()
        }
    };

    Ok(Json(educations))
}
