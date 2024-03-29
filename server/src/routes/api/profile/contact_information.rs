use axum::extract::{Path, State};
use axum::Json;
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::AppState;

/// Asynchronously retrieves public contact information for a user.
///
/// # Arguments
///
/// * `state` - The application state containing service instances.
/// * `session` - The session information for the current user.
/// * `identifier` - The username of the user whose contact information is to be retrieved.
///
/// # Returns
///
/// Returns a JSON representation of the public contact information associated with the user.
/// The outcome differs based on whether the current user is the owner of the profile or not.
/// If the current user is the owner, it retrieves authenticated contact information.
/// If the current user is not the owner, it retrieves public contact information.
///
/// # Errors
///
/// Returns an `AppError` if there is an error during the retrieval process.
///
pub async fn get_public_contact_information(
    State(state): State<AppState>,
    session: Session,
    Path(identifier): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let optional_user = state.user_service.check_user_optional(&session).await?;
    let user = state.user_service.get_user_by_username(identifier).await?;

    let contact_information = match optional_user {
        Some(logged_in_user) if logged_in_user.id == user.id => {
            let contact_information = state
                .contact_information_service
                .get_authenticated_contact_information(logged_in_user.id)
                .await?;
            serde_json::to_value(&contact_information).unwrap()
        }
        _ => {
            let contact_information = state
                .contact_information_service
                .get_public_contact_information(user.id, None)
                .await?;
            serde_json::to_value(&contact_information).unwrap()
        }
    };

    Ok(Json(contact_information))
}
