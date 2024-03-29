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

    /// Asynchronously retrieves experiences associated with an authenticated user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the authenticated user whose experiences are to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `AuthExperienceModel` instances representing the experiences associated with the authenticated user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
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

    /// Asynchronously retrieves public experiences associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose public experiences are to be retrieved.
    /// * `limit` - An optional limit on the number of experiences to retrieve. If not provided, retrieves all experiences.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `PublicExperienceModel` instances representing the public experiences associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
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

    /// Asynchronously checks if a user owns a specific experience.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to check ownership.
    /// * `experience_id` - The ID of the experience to check ownership against.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<bool>`. If the user owns the experience, it returns `Some(true)`,
    /// if the user does not own the experience, it returns `Some(false)`. If the experience is not found, it returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    /// Returns an `AppError::NotFound` with an error message if the experience is not found.
    ///
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

    /// Asynchronously deletes an experience associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user who owns the experience to be deleted.
    /// * `experience_id` - The ID of the experience to be deleted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the delete operation. If the experience is successfully deleted,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the delete operation.
    ///
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

    /// Asynchronously updates an experience associated with a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user who owns the experience to be updated.
    /// * `experience_id` - The ID of the experience to be updated.
    /// * `payload` - An `UpdateExperiencePayload` containing the updated data for the experience.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the update operation. If the experience is successfully updated,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the update operation.
    ///
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

    /// Asynchronously creates a new experience for a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the experience is to be created.
    /// * `payload` - An `AddExperiencePayload` containing the details of the experience to be created.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the ID of the newly created experience if the operation is successful.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the insert operation.
    ///
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

    /// Validates the experience type.
    ///
    /// # Arguments
    ///
    /// * `experience_type` - An optional reference to the experience type.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the experience type is valid (`Ok(true)`) or invalid (`Err(AppError::BadRequest)`).
    ///
    /// # Errors
    ///
    /// Returns an `AppError::BadRequest` with an error message if the experience type is invalid.
    ///
    pub fn validate_experience_type(
        &self,
        experience_type: &Option<String>,
    ) -> Result<bool, AppError> {
        if let Some(exp_type) = experience_type {
            if exp_type.len() == 0 {
                return Ok(true);
            };

            let exp_type = ExperienceType::from_str(exp_type);
            if exp_type.is_none() {
                return Err(AppError::BadRequest {
                    error: Some("Invalid experience type".to_string()),
                });
            };
        }

        Ok(true)
    }

    /// Asynchronously retrieves the count of experiences associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose experience count is to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the count of experiences associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
    pub async fn get_experience_count(&self, user_id: i32) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM experiences WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|id| id.0)
    }

    /// Asynchronously retrieves all experiences associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose experiences are to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `ExperienceModel` instances representing all experiences associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
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

    /// Asynchronously deletes an experience from the database for administrative purposes.
    ///
    /// # Arguments
    ///
    /// * `experience_id` - The ID of the experience to be deleted.
    ///
    /// # Returns
    ///
    /// Returns `Result<(), AppError>` indicating the outcome of the delete operation. If the experience is successfully deleted, it returns `Ok(())`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the delete operation.
    ///
    pub async fn admin_delete_experience(&self, experience_id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM experiences WHERE id = $1")
            .bind(experience_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|_| ())
    }

    /// Asynchronously checks if an experience exists in the database.
    ///
    /// # Arguments
    ///
    /// * `experience_id` - The ID of the experience to check for existence.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<bool>`. If the experience exists, it returns `Some(true)`.
    /// If the experience does not exist, it returns `Some(false)`. If there is an error while querying the database, it returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    /// Returns an `AppError::NotFound` with an error message if the experience is not found.
    ///
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
