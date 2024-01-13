use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use tower_sessions::Session;

use crate::auth::check_user::check_user;
use crate::models::contact_information::ContactInformationModel;
use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
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
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let contact_information = sqlx::query_as::<_, ContactInformationModel>(
        "SELECT * FROM contact_information WHERE user_id = $1",
    )
    .bind(user.id)
    .fetch_all(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&contact_information).unwrap())
        .unwrap())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddContactInformationPayload {
    contact_type: String,
    value: String,
}

pub async fn add_contact_information(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AddContactInformationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let count =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM contact_information WHERE user_id = $1")
            .bind(user.id)
            .bind(&payload.contact_type)
            .bind(&payload.value)
            .fetch_one(&*state.db)
            .await
            .map_err(|_| AppError::InternalError)?;

    if count.0 >= 50 {
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

    let contact_info_result = sqlx::query_as::<_, ContactInformationModel>(
        "SELECT * FROM contact_information WHERE user_id = $1 AND type_field = $2 AND value = $3 LIMIT 1",
    )
    .bind(user.id)
    .bind(&payload.contact_type)
    .bind(&payload.value)
    .fetch_all(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    if !contact_info_result.is_empty() {
        return Err(AppError::DataConflict {
            error: "Contact information already exists".to_string(),
        });
    }

    let new_contact_info = sqlx::query_as::<_, (i32,)>("INSERT INTO contact_information (user_id, type_field, value) VALUES ($1, $2, $3) RETURNING id")
        .bind(user.id)
        .bind(payload.contact_type)
        .bind(payload.value)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::CREATED {
        id: Some(new_contact_info.0),
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateContactInformationPayload {
    id: i32,
    contact_type: String,
    value: String,
}

pub async fn update_contact_information(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<UpdateContactInformationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let entry_does_exist = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM contact_information WHERE id = $1 AND user_id = $2",
    )
    .bind(payload.id)
    .bind(user.id)
    .fetch_one(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)
    .map_or_else(|_| None, |count| Some(count.0 > 0));

    if !entry_does_exist.unwrap() {
        return Err(AppError::NotFound {
            error: "Contact information not found".to_string(),
        });
    }

    let contact_type = ContactType::from_str(&payload.contact_type);
    if contact_type.is_none() {
        return Err(AppError::BadRequest {
            error: Some("Invalid contact type".to_string()),
        });
    }

    sqlx::query(
        "UPDATE contact_information SET type_field = $1, value = $2 WHERE id = $3 AND user_id = $4",
    )
    .bind(payload.contact_type)
    .bind(payload.value)
    .bind(payload.id)
    .bind(user.id)
    .execute(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::UPDATED)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteContactInformationPayload {
    id: i32,
}

pub async fn delete_contact_information(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<DeleteContactInformationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    sqlx::query("DELETE FROM contact_information WHERE id = $1 AND user_id = $2")
        .bind(payload.id)
        .bind(user.id)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::DELETED)
}
