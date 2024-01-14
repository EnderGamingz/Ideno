use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use tower_sessions::Session;

use crate::auth::check_user::check_user;
use crate::models::experience::ExperienceModel;
use crate::models::user::UserModel;
use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::AppState;

enum ExperienceType {
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
    fn from_str(s: &str) -> Option<ExperienceType> {
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

pub async fn get_experiences(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let experiences =
        sqlx::query_as::<_, ExperienceModel>("SELECT * FROM experiences WHERE user_id = $1")
            .bind(user.id)
            .fetch_all(&*state.db)
            .await
            .map_err(|_| AppError::InternalError)?;

    Ok(Response::builder()
        .status(200)
        .body(serde_json::to_string(&experiences).unwrap())
        .unwrap())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddExperiencePayload {
    company: String,
    title: String,
    start_date: Option<String>,
    end_date: Option<String>,
    exp_type: Option<String>,
    description: Option<String>,
}

pub async fn add_experience(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AddExperiencePayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let count = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM experiences WHERE user_id = $1")
        .bind(user.id)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    if count.0 >= 50 {
        return Err(AppError::DataConflict {
            error: "Experiences limit reached".to_string(),
        });
    }

    validate_experience_type(&payload.exp_type)?;

    let new_experience = sqlx::query_as::<_, (i64,)>(
        "INSERT INTO experiences (company, title, start_date, end_date, exp_type, description, user_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
    )
    .bind(payload.company)
    .bind(payload.title)
    .bind(payload.start_date)
    .bind(payload.end_date)
    .bind(payload.exp_type)
    .bind(payload.description)
    .bind(user.id)
    .fetch_one(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::CREATED {
        id: Some(new_experience.0),
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateExperiencePayload {
    company: String,
    title: String,
    start_date: Option<String>,
    end_date: Option<String>,
    exp_type: Option<String>,
    description: Option<String>,
}

pub async fn update_experience(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateExperiencePayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;
    experience_exists_for_user_id(&state, id, &user).await?;

    validate_experience_type(&payload.exp_type)?;

    sqlx::query(
        "UPDATE experiences SET company = $1, title = $2, start_date = $3, end_date = $4, exp_type = $5, description = $6 WHERE id = $7 AND user_id = $8",
    )
    .bind(payload.company)
    .bind(payload.title)
    .bind(payload.start_date)
    .bind(payload.end_date)
    .bind(payload.exp_type)
    .bind(payload.description)
    .bind(id)
    .bind(user.id)
    .execute(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::UPDATED)
}

pub async fn delete_experience(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    experience_exists_for_user_id(&state, id, &user).await?;

    sqlx::query("DELETE FROM experiences WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user.id)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::DELETED)
}

fn validate_experience_type(experience_type: &Option<String>) -> Result<(), AppError> {
    if let Some(exp_type) = experience_type {
        let exp_type = ExperienceType::from_str(exp_type);
        if exp_type.is_none() {
            return Err(AppError::BadRequest {
                error: Some("Invalid experience type".to_string()),
            });
        };
    }
    Ok(())
}

async fn experience_exists_for_user_id(
    state: &AppState,
    payload: i32,
    user: &UserModel,
) -> Result<Option<bool>, AppError> {
    let flag = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM experiences WHERE id = $1 AND user_id = $2",
    )
    .bind(payload)
    .bind(user.id)
    .fetch_one(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)
    .map_or_else(|_| None, |count| Some(count.0 > 0));

    if !flag.unwrap() {
        return Err(AppError::NotFound {
            error: "Experience not found".to_string(),
        });
    }

    Ok(flag)
}
