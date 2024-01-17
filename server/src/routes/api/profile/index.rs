use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::models::certification::PublicCertificationModel;
use crate::models::contact_information::PublicContactInformationModel;
use crate::models::education::PublicEducationModel;
use crate::models::experience::PublicExperienceModel;
use crate::models::profile::{PublicProfileModel, PublicProfileResponse};
use crate::response::error_handling::AppError;
use crate::AppState;

pub async fn get_public_profile(
    State(state): State<AppState>,
    Path(identifier): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = sqlx::query_as::<_, (i64,)>("SELECT id FROM users where username = ?")
        .bind(identifier)
        .fetch_optional(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?
        .map(|row| row.0);

    if user_id.is_none() {
        return Err(AppError::NotFound {
            error: "Profile not Found".to_string(),
        });
    }

    let found_profile = sqlx::query_as::<_, PublicProfileModel>(
        "SELECT
                first_name,
                last_name,
                pronouns,
                headline,
                country,
                city,
                bio
            FROM profiles where user_id = ?",
    )
    .bind(user_id.unwrap())
    .fetch_one(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    let certifications = sqlx::query_as::<_, PublicCertificationModel>(
        "SELECT
                name,
                organization,
                issue_date,
                expiration_date,
                credential_id,
                credential_url
              FROM certification
              WHERE user_id = ?",
    )
    .bind(user_id.unwrap())
    .fetch_all(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    let educations = sqlx::query_as::<_, PublicEducationModel>(
        "SELECT
                school,
                degree,
                field,
                start_date,
                end_date
            FROM educations
            WHERE user_id = ?",
    )
    .fetch_all(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    let experiences = sqlx::query_as::<_, PublicExperienceModel>(
        "SELECT
                company,
                title,
                start_date,
                end_date,
                exp_type,
                description
            FROM experiences
            WHERE user_id = ?",
    )
    .fetch_all(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    let contact_information = sqlx::query_as::<_, PublicContactInformationModel>(
        "SELECT
                type_field,
                value
            FROM contact_information
            WHERE user_id = ?",
    )
    .fetch_all(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

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
