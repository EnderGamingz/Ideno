use axum::extract::State;
use axum::Json;
use tower_sessions::Session;

use crate::models::profile::PublicProfileModel;
use crate::response::error_handling::AppError;
use crate::AppState;

pub async fn get_profile(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = state.user_service.check_user(&session).await?;

    let profile = state.profile_service.get_public_profile(user.id).await?;

    Ok(Json(serde_json::to_value(profile).unwrap()))
}

/// Asynchronously updates the public profile of a user.
///
/// # Arguments
///
/// * `state` - The application state containing service instances.
/// * `session` - The session information for the current user.
/// * `payload` - A JSON payload containing the updated public profile information.
///
/// # Returns
///
/// Returns a JSON representation of the updated public profile.
///
/// # Note
///
/// The function manages pronouns by checking if a pronoun template exists in the payload.
/// If a template exists, it checks for a corresponding pronoun in the template mapping.
/// If a corresponding pronoun is found, it replaces the template with the full pronoun.
/// If no corresponding pronoun is found or if there is no template, it retains the original pronoun.
///
/// # Errors
///
/// Returns an `AppError` if there is an error during the update process.
///
pub async fn update_profile(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<PublicProfileModel>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = state.user_service.check_user(&session).await?;

    const PRONOUNS_MAP: [(&str, &str); 3] =
        [("he", "he/him"), ("she", "she/her"), ("they", "they/them")];
    // Checks for payload pronouns if a template exists if not, just returns the pronouns
    let personal_pronouns = if let Some(user_pronouns) = payload.pronouns {
        PRONOUNS_MAP
            .into_iter()
            .find(|(key, _)| key == &user_pronouns)
            .map(|(_, value)| Some(value.to_string()))
            .unwrap_or(Some(user_pronouns))
    } else {
        None
    };

    let updated_payload = PublicProfileModel {
        pronouns: personal_pronouns,
        ..payload
    };

    let profile = state
        .profile_service
        .update_profile(user.id, updated_payload)
        .await?;

    Ok(Json(serde_json::to_value(&profile).unwrap()))
}
