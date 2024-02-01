use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::services::education_service::{AddEducationPayload, UpdateEducationPayload};
use crate::AppState;

pub async fn get_educations(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = state.user_service.check_user(&session).await?;

    let educations = state.education_service.get_all_educations(user.id).await?;

    Ok(Json(serde_json::to_value(educations).unwrap()))
}

pub async fn add_education(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AddEducationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;

    let count = state.education_service.get_education_count(user.id).await?;

    if count >= 50 {
        return Err(AppError::DataConflict {
            error: "Education limit reached".to_string(),
        });
    }

    let created_education = state
        .education_service
        .create_education(user.id, payload)
        .await?;

    Ok(AppSuccess::CREATED {
        id: Some(created_education),
    })
}

pub async fn update_education(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateEducationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;

    state
        .education_service
        .user_owns_education(user.id, id)
        .await?;

    state
        .education_service
        .update_education(user.id, id, payload)
        .await?;

    Ok(AppSuccess::UPDATED)
}

pub async fn delete_education(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;

    state
        .education_service
        .user_owns_education(user.id, id)
        .await?;

    state
        .education_service
        .delete_education(user.id, id)
        .await?;

    Ok(AppSuccess::DELETED)
}
