use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::services::contact_information_service::{
    AddContactInformationPayload, UpdateContactInformationPayload,
};
use crate::AppState;

enum ContactType {
    Email,
    Phone,
    Website,
    LinkedIn,
    GitHub,
    Twitter,
    Facebook,
    Instagram,
}

impl ContactType {
    fn from_str(contact_type: &str) -> Option<Self> {
        match contact_type {
            "email" => Some(ContactType::Email),
            "phone" => Some(ContactType::Phone),
            "website" => Some(ContactType::Website),
            "linkedin" => Some(ContactType::LinkedIn),
            "github" => Some(ContactType::GitHub),
            "twitter" => Some(ContactType::Twitter),
            "facebook" => Some(ContactType::Facebook),
            "instagram" => Some(ContactType::Instagram),
            _ => None,
        }
    }
}

pub async fn get_contact_information(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = state.user_service.check_user(&session).await?;

    let contact_information = state
        .contact_information_service
        .get_all_contact_information(user.id)
        .await?;

    Ok(Json(serde_json::to_value(contact_information).unwrap()))
}

pub async fn add_contact_information(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AddContactInformationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;

    let count = state
        .contact_information_service
        .get_contact_information_count(user.id)
        .await?;

    if count >= 50 {
        return Err(AppError::DataConflict {
            error: "Contact information limit reached".to_string(),
        });
    }

    let contact_type = ContactType::from_str(&payload.contact_type);
    if contact_type.is_none() {
        return Err(AppError::BadRequest {
            error: Some("Invalid contact type".to_string()),
        });
    }

    let contact_information_exists = state
        .contact_information_service
        .get_existing_contact_information(user.id, &payload)
        .await?;

    if contact_information_exists {
        return Err(AppError::DataConflict {
            error: "Contact information already exists".to_string(),
        });
    }

    let created_id = state
        .contact_information_service
        .create_contact_information(user.id, payload)
        .await?;

    Ok(AppSuccess::CREATED {
        id: Some(created_id),
    })
}

pub async fn update_contact_information(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateContactInformationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;

    state
        .contact_information_service
        .user_owns_contact_information(user.id, id)
        .await?;

    let contact_type = ContactType::from_str(&payload.contact_type);
    if contact_type.is_none() {
        return Err(AppError::BadRequest {
            error: Some("Invalid contact type".to_string()),
        });
    }

    state
        .contact_information_service
        .update_contact_information(user.id, id, payload)
        .await?;

    Ok(AppSuccess::UPDATED)
}

pub async fn delete_contact_information(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;

    state
        .contact_information_service
        .user_owns_contact_information(user.id, id)
        .await?;

    state
        .contact_information_service
        .delete_contact_information(user.id, id)
        .await?;

    Ok(AppSuccess::DELETED)
}
