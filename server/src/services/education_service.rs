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

    /// Asynchronously retrieves authenticated educations associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the authenticated user whose educations are to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `AuthEducationModel` instances representing the authenticated educations associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
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
              WHERE user_id = ?
              ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously retrieves public educations associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose public educations are to be retrieved.
    /// * `limit` - An optional limit on the number of educations to retrieve. If not provided, retrieves all educations.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `PublicEducationModel` instances representing the public educations associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
    pub async fn get_public_educations(
        &self,
        user_id: i32,
        limit: Option<i32>,
    ) -> Result<Vec<PublicEducationModel>, AppError> {
        let limit = limit.unwrap_or(-1);
        sqlx::query_as::<_, PublicEducationModel>(
            "SELECT
                school,
                degree,
                field,
                start_date,
                end_date
              FROM educations
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

    /// Asynchronously checks if a user owns a specific education.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to check ownership.
    /// * `education_id` - The ID of the education to check ownership against.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<bool>`. If the user owns the education, it returns `Some(true)`.
    /// If the user does not own the education, it returns `Some(false)`. If the education is not found, it returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    /// Returns an `AppError::NotFound` with an error message if the education is not found.
    ///
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

    /// Asynchronously deletes an education associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user who owns the education to be deleted.
    /// * `education_id` - The ID of the education to be deleted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the delete operation. If the education is successfully deleted,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the delete operation.
    ///
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

    /// Asynchronously updates an education associated with a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user who owns the education to be updated.
    /// * `education_id` - The ID of the education to be updated.
    /// * `payload` - An `UpdateEducationPayload` containing the updated data for the education.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the update operation. If the education is successfully updated,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the update operation.
    ///
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

    /// Asynchronously creates a new education for a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the education is to be created.
    /// * `payload` - An `AddEducationPayload` containing the details of the education to be created.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the ID of the newly created education if the operation is successful.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the insert operation.
    ///
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

    /// Asynchronously retrieves the count of educations associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose education count is to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the count of educations associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
    pub async fn get_education_count(&self, user_id: i32) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM educations WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|count| count.0)
    }

    /// Asynchronously retrieves all educations associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose educations are to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `EducationModel` instances representing all educations associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
    pub async fn get_all_educations(&self, user_id: i32) -> Result<Vec<EducationModel>, AppError> {
        sqlx::query_as::<_, EducationModel>(
            "SELECT * FROM educations WHERE user_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously deletes an education from the database for administrative purposes.
    ///
    /// # Arguments
    ///
    /// * `education_id` - The ID of the education to be deleted.
    ///
    /// # Returns
    ///
    /// Returns `Result<(), AppError>` indicating the outcome of the delete operation. If the education is successfully deleted,
    /// it returns `Ok(())`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the delete operation.
    ///
    pub async fn admin_delete_education(&self, education_id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM educations WHERE id = $1")
            .bind(education_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|_| ())
    }

    /// Asynchronously checks if an education exists in the database.
    ///
    /// # Arguments
    ///
    /// * `education_id` - The ID of the education to check for existence.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<bool>`. If the education exists, it returns `Some(true)`.
    /// If the education does not exist, it returns `Some(false)`. If there is an error while querying the database, it returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    /// Returns an `AppError::NotFound` with an error message if the education is not found.
    ///
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
