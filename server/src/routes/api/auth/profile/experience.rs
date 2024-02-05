use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::services::experience_service::{AddExperiencePayload, UpdateExperiencePayload};
use crate::AppState;

#[derive(PartialEq, Debug)]
pub enum ExperienceType {
    FullTime,
    PartTime,
    SelfEmployed,
    Freelance,
    Contract,
    Internship,
    Volunteering,
    Seasonal,
    Apprenticeship,
    Other,
}

impl ExperienceType {
    pub(crate) fn from_str(s: &str) -> Option<ExperienceType> {
        match s {
            "Full Time" => Some(ExperienceType::FullTime),
            "Part Time" => Some(ExperienceType::PartTime),
            "Self Employed" => Some(ExperienceType::SelfEmployed),
            "Freelance" => Some(ExperienceType::Freelance),
            "Contract" => Some(ExperienceType::Contract),
            "Internship" => Some(ExperienceType::Internship),
            "Volunteering" => Some(ExperienceType::Volunteering),
            "Seasonal" => Some(ExperienceType::Seasonal),
            "Apprenticeship" => Some(ExperienceType::Apprenticeship),
            "Other" => Some(ExperienceType::Other),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExperienceType;

    #[test]
    fn test_from_str() {
        assert_eq!(ExperienceType::from_str("Full Time"), Some(ExperienceType::FullTime));
        assert_eq!(ExperienceType::from_str("Part Time"), Some(ExperienceType::PartTime));
        assert_eq!(ExperienceType::from_str("Self Employed"), Some(ExperienceType::SelfEmployed));
        assert_eq!(ExperienceType::from_str("Freelance"), Some(ExperienceType::Freelance));
        assert_eq!(ExperienceType::from_str("Contract"), Some(ExperienceType::Contract));
        assert_eq!(ExperienceType::from_str("Internship"), Some(ExperienceType::Internship));
        assert_eq!(ExperienceType::from_str("Volunteering"), Some(ExperienceType::Volunteering));
        assert_eq!(ExperienceType::from_str("Seasonal"), Some(ExperienceType::Seasonal));
        assert_eq!(ExperienceType::from_str("Apprenticeship"), Some(ExperienceType::Apprenticeship));
        assert_eq!(ExperienceType::from_str("Other"), Some(ExperienceType::Other));

        // Test with a string that is not a contact type
        assert_eq!(ExperienceType::from_str("invalid"), None);
    }
}

pub async fn get_experiences(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = state.user_service.check_user(&session).await?;

    let experiences = state
        .experience_service
        .get_all_experiences(user.id)
        .await?;

    Ok(Json(serde_json::to_value(experiences).unwrap()))
}

pub async fn add_experience(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AddExperiencePayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;

    let count = state
        .experience_service
        .get_experience_count(user.id)
        .await?;

    if count >= 50 {
        return Err(AppError::DataConflict {
            error: "Experiences limit reached".to_string(),
        });
    }

    state
        .experience_service
        .validate_experience_type(&payload.exp_type)?;

    let new_experience = state
        .experience_service
        .create_experience(user.id, payload)
        .await?;

    Ok(AppSuccess::CREATED {
        id: Some(new_experience),
    })
}

pub async fn update_experience(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateExperiencePayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;
    state
        .experience_service
        .user_owns_experience(user.id, id)
        .await?;

    state
        .experience_service
        .validate_experience_type(&payload.exp_type)?;

    state
        .experience_service
        .update_experience(user.id, id, payload)
        .await?;

    Ok(AppSuccess::UPDATED)
}

pub async fn delete_experience(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;
    state
        .experience_service
        .user_owns_experience(user.id, id)
        .await?;

    state
        .experience_service
        .delete_experience(user.id, id)
        .await?;

    Ok(AppSuccess::DELETED)
}
