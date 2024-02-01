use crate::models::education::{AuthEducationModel, EducationModel, PublicEducationModel};
use crate::response::error_handling::AppError;
use crate::{IdenoDBResult, IdenoPool};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddEducationPayload {
    pub school: String,
    pub degree: Option<String>,
    pub field: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateEducationPayload {
    pub school: String,
    pub degree: Option<String>,
    pub field: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Clone)]
pub struct EducationService {
    db_pool: IdenoPool,
}

impl EducationService {
    pub fn new(db_pool: IdenoPool) -> Self {
        EducationService { db_pool }
    }

    pub async fn get_authenticated_educations(
        &self,
        user_id: i32,
    ) -> Result<Vec<AuthEducationModel>, AppError> {
        sqlx::query_as::<_, AuthEducationModel>(
            "SELECT
                id,
                school,
                degree,
                field,
                start_date,
                end_date
              FROM educations
              WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    pub async fn get_public_educations(
        &self,
        user_id: i32,
    ) -> Result<Vec<PublicEducationModel>, AppError> {
        sqlx::query_as::<_, PublicEducationModel>(
            "SELECT
                school,
                degree,
                field,
                start_date,
                end_date
              FROM educations
              WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    pub async fn user_owns_education(
        &self,
        user_id: i32,
        education_id: i32,
    ) -> Result<Option<bool>, AppError> {
        let flag = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM educations WHERE id = $1 AND user_id = $2",
        )
        .bind(education_id)
        .bind(user_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
        .map_or_else(|_| None, |count| Some(count.0 > 0));
        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Education not found".to_string(),
            }),
        }
    }

    pub async fn delete_education(
        &self,
        user_id: i32,
        education_id: i32,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("DELETE FROM educations WHERE id = $1 AND user_id = $2")
            .bind(education_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn update_education(
        &self,
        user_id: i32,
        education_id: i32,
        payload: UpdateEducationPayload,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE educations SET school = $1, degree = $2, field = $3, start_date = $4, end_date = $5 WHERE id = $6 AND user_id = $7")
            .bind(payload.school)
            .bind(payload.degree)
            .bind(payload.field)
            .bind(payload.start_date)
            .bind(payload.end_date)
            .bind(education_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn create_education(
        &self,
        user_id: i32,
        payload: AddEducationPayload,
    ) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("INSERT INTO educations (user_id, school, degree, field, start_date, end_date) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
            .bind(user_id)
            .bind(payload.school)
            .bind(payload.degree)
            .bind(payload.field)
            .bind(payload.start_date)
            .bind(payload.end_date)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|id| id.0)
    }

    pub async fn get_education_count(&self, user_id: i32) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM educations WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|count| count.0)
    }

    pub async fn get_all_educations(&self, user_id: i32) -> Result<Vec<EducationModel>, AppError> {
        sqlx::query_as::<_, EducationModel>("SELECT * FROM educations WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn admin_delete_education(&self, education_id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM educations WHERE id = $1")
            .bind(education_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|_| ())
    }

    pub async fn education_exists(&self, education_id: i32) -> Result<Option<bool>, AppError> {
        let flag = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM educations WHERE id = $1")
            .bind(education_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map_or_else(|_| None, |count| Some(count.0 > 0));

        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Education not found".to_string(),
            }),
        }
    }
}
