use crate::models::experience::{AuthExperienceModel, ExperienceModel, PublicExperienceModel};
use crate::response::error_handling::AppError;
use crate::routes::api::auth::profile::experience::ExperienceType;
use crate::{IdenoDBResult, IdenoPool};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddExperiencePayload {
    pub company: String,
    pub title: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub exp_type: Option<String>,
    pub description: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateExperiencePayload {
    pub company: String,
    pub title: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub exp_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct ExperienceService {
    db_pool: IdenoPool,
}

impl ExperienceService {
    pub fn new(db_pool: IdenoPool) -> Self {
        ExperienceService { db_pool }
    }

    pub async fn get_authenticated_experiences(
        &self,
        user_id: i32,
    ) -> Result<Vec<AuthExperienceModel>, AppError> {
        sqlx::query_as::<_, AuthExperienceModel>(
            "SELECT
                id,
                company,
                title,
                start_date,
                end_date,
                exp_type,
                description
              FROM experiences
              WHERE user_id = ?
              ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    pub async fn get_public_experiences(
        &self,
        user_id: i32,
        limit: Option<i32>,
    ) -> Result<Vec<PublicExperienceModel>, AppError> {
        let limit = limit.unwrap_or(-1);
        sqlx::query_as::<_, PublicExperienceModel>(
            "SELECT
                company,
                title,
                start_date,
                end_date,
                exp_type,
                description
              FROM experiences
              WHERE user_id = $1
              ORDER BY created_at DESC
              LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    pub async fn user_owns_experience(
        &self,
        user_id: i32,
        experience_id: i32,
    ) -> Result<Option<bool>, AppError> {
        let flag = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM experiences WHERE id = $1 AND user_id = $2",
        )
        .bind(experience_id)
        .bind(user_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
        .map_or_else(|_| None, |count| Some(count.0 > 0));
        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Experience not found".to_string(),
            }),
        }
    }

    pub async fn delete_experience(
        &self,
        user_id: i32,
        experience_id: i32,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("DELETE FROM experiences WHERE id = $1 AND user_id = $2")
            .bind(experience_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn update_experience(
        &self,
        user_id: i32,
        experience_id: i32,
        payload: UpdateExperiencePayload,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query(
            "UPDATE experiences SET company = $1, title = $2, start_date = $3, end_date = $4, exp_type = $5, description = $6 WHERE id = $7 AND user_id = $8",
        )
            .bind(payload.company)
            .bind(payload.title)
            .bind(payload.start_date)
            .bind(payload.end_date)
            .bind(payload.exp_type)
            .bind(payload.description)
            .bind(experience_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn create_experience(
        &self,
        user_id: i32,
        payload: AddExperiencePayload,
    ) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>(
            "INSERT INTO experiences (company, title, start_date, end_date, exp_type, description, user_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
        )
            .bind(payload.company)
            .bind(payload.title)
            .bind(payload.start_date)
            .bind(payload.end_date)
            .bind(payload.exp_type)
            .bind(payload.description)
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|id| id.0)
    }

    pub fn validate_experience_type(
        &self,
        experience_type: &Option<String>,
    ) -> Result<bool, AppError> {
        if let Some(exp_type) = experience_type {
            let exp_type = ExperienceType::from_str(exp_type);
            if exp_type.is_none() {
                return Err(AppError::BadRequest {
                    error: Some("Invalid experience type".to_string()),
                });
            };
        }

        Ok(true)
    }

    pub async fn get_experience_count(&self, user_id: i32) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM experiences WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|id| id.0)
    }

    pub async fn get_all_experiences(
        &self,
        user_id: i32,
    ) -> Result<Vec<ExperienceModel>, AppError> {
        sqlx::query_as::<_, ExperienceModel>(
            "SELECT * FROM experiences WHERE user_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    pub async fn admin_delete_experience(&self, experience_id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM experiences WHERE id = $1")
            .bind(experience_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|_| ())
    }

    pub async fn experience_exists(&self, experience_id: i32) -> Result<Option<bool>, AppError> {
        let flag = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM experiences WHERE id = $1")
            .bind(experience_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map_or_else(|_| None, |count| Some(count.0 > 0));

        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Experience not found".to_string(),
            }),
        }
    }
}
