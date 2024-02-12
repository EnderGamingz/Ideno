use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::models::profile::PublicProfileResponse;
use crate::response::error_handling::AppError;
use crate::AppState;

pub async fn get_public_profile(
    State(state): State<AppState>,
    Path(identifier): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.get_user_by_username(identifier).await?;
    let found_profile = state.profile_service.get_public_profile(user.id).await?;

    let certifications = state
        .certification_service
        .get_public_certifications(user.id, Some(3))
        .await?;
    let educations = state
        .education_service
        .get_public_educations(user.id, Some(3))
        .await?;
    let experiences = state
        .experience_service
        .get_public_experiences(user.id, Some(3))
        .await?;
    let contact_information = state
        .contact_information_service
        .get_public_contact_information(user.id, Some(4))
        .await?;

    let response = PublicProfileResponse {
        profile: found_profile,
        certification: certifications,
        education: educations,
        experience: experiences,
        contact_information,
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&response).unwrap())
        .unwrap())
}
