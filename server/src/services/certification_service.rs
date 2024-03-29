use crate::models::certification::{
    AuthCertificationModel, CertificationModel, PublicCertificationModel,
};
use crate::response::error_handling::AppError;
use crate::{IdenoDBResult, IdenoPool};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddCertificationPayload {
    pub name: String,
    pub organization: String,
    pub issue_date: Option<String>,
    pub expiration_date: Option<String>,
    pub credential_id: Option<String>,
    pub credential_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateCertificationPayload {
    name: String,
    organization: String,
    issue_date: Option<String>,
    expiration_date: Option<String>,
    credential_id: Option<String>,
    credential_url: Option<String>,
}

#[derive(Clone)]
pub struct CertificationService {
    db_pool: IdenoPool,
}

impl CertificationService {
    pub fn new(db_pool: IdenoPool) -> Self {
        CertificationService { db_pool }
    }

    /// Asynchronously retrieves authenticated certifications associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the authenticated certifications are being retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `AuthCertificationModel` representing authenticated certifications associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the query.
    ///
    pub async fn get_authenticated_certifications(
        &self,
        user_id: i32,
    ) -> Result<Vec<AuthCertificationModel>, AppError> {
        sqlx::query_as::<_, AuthCertificationModel>(
            "SELECT
                id,
                name,
                organization,
                issue_date,
                expiration_date,
                credential_id,
                credential_url
              FROM certification
              WHERE user_id = ?
              ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously retrieves public certifications associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the public certifications are being retrieved.
    /// * `limit` - An optional limit on the number of certifications to retrieve. If not provided, all certifications are fetched.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `PublicCertificationModel` representing public certifications associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the query.
    ///
    pub async fn get_public_certifications(
        &self,
        user_id: i32,
        limit: Option<i32>,
    ) -> Result<Vec<PublicCertificationModel>, AppError> {
        let limit = limit.unwrap_or(-1);
        sqlx::query_as::<_, PublicCertificationModel>(
            "SELECT
                name,
                organization,
                issue_date,
                expiration_date,
                credential_id,
                credential_url
              FROM certification
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

    /// Asynchronously checks if a user owns a specific certification.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose ownership of the certification is being checked.
    /// * `certification_id` - The ID of the certification being checked for ownership.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an optional boolean value indicating whether the user owns the certification.
    /// - `Some(true)` if the user owns the certification.
    /// - `Some(false)` if the user does not own the certification.
    /// - `None` if there was an error while executing the query.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the query.
    /// Returns an `AppError::NotFound` if the certification with the provided ID does not exist.
    ///
    pub async fn user_owns_certification(
        &self,
        user_id: i32,
        certification_id: i32,
    ) -> Result<Option<bool>, AppError> {
        let flag = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM certification WHERE id = $1 AND user_id = $2",
        )
        .bind(certification_id)
        .bind(user_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
        .map_or_else(|_| None, |count| Some(count.0 > 0));
        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Certification not found".to_string(),
            }),
        }
    }

    /// Asynchronously deletes a certification entry associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user who owns the certification entry to be deleted.
    /// * `certification_id` - The ID of the certification entry to be deleted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the delete operation. If the certification entry is successfully deleted,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the delete operation.
    ///
    pub async fn delete_certification(
        &self,
        user_id: i32,
        certification_id: i32,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("DELETE FROM certification WHERE id = $1 AND user_id = $2")
            .bind(certification_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously updates a certification entry associated with a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user who owns the certification entry to be updated.
    /// * `certification_id` - The ID of the certification entry to be updated.
    /// * `payload` - An `UpdateCertificationPayload` containing the updated certification details.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the update operation. If the certification entry is successfully updated,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the update operation.
    ///
    pub async fn update_certification(
        &self,
        user_id: i32,
        certification_id: i32,
        payload: UpdateCertificationPayload,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE certification SET name = $1, organization = $2, issue_date = $3, expiration_date = $4, credential_id = $5, credential_url = $6 WHERE id = $7 AND user_id = $8")
            .bind(payload.name)
            .bind(payload.organization)
            .bind(payload.issue_date)
            .bind(payload.expiration_date)
            .bind(payload.credential_id)
            .bind(payload.credential_url)
            .bind(certification_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously creates a new certification entry associated with a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the certification is being created.
    /// * `payload` - An `AddCertificationPayload` containing the details of the certification to be created.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the ID of the newly created certification entry if successful.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the insert operation.
    ///
    pub async fn create_certification(
        &self,
        user_id: i32,
        payload: AddCertificationPayload,
    ) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64, )>
            ("INSERT INTO certification (user_id, name, organization, issue_date, expiration_date, credential_id, credential_url) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id")
            .bind(user_id)
            .bind(payload.name)
            .bind(payload.organization)
            .bind(payload.issue_date)
            .bind(payload.expiration_date)
            .bind(payload.credential_id)
            .bind(payload.credential_url)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|id| id.0)
    }

    /// Asynchronously retrieves the count of certification entries associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the count of certification entries is being retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the count of certification entries associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the query.
    ///
    pub async fn get_certification_count(&self, user_id: i32) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM certification WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|count| count.0)
    }

    /// Asynchronously retrieves all certification entries associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the certification entries are being retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `CertificationModel` representing all certification entries associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the query.
    ///
    pub async fn get_all_certifications(
        &self,
        user_id: i32,
    ) -> Result<Vec<CertificationModel>, AppError> {
        sqlx::query_as::<_, CertificationModel>("SELECT * FROM certification WHERE user_id = $1 ORDER BY created_at DESC")
            .bind(user_id)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously deletes a certification entry from the database foe administrative purposes.
    ///
    /// # Arguments
    ///
    /// * `certification_id` - The ID of the certification entry to be deleted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the delete operation. If the operation is successful, it returns `Ok(())`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the delete operation.
    ///
    pub async fn admin_delete_certification(&self, certification_id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM certification WHERE id = $1")
            .bind(certification_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|_| ())
    }

    /// Asynchronously checks if a certification entry exists in the database.
    ///
    /// # Arguments
    ///
    /// * `certification_id` - The ID of the certification entry to be checked.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an optional boolean value.
    /// If the certification entry exists, it returns `Some(true)`, otherwise `Some(false)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the query.
    /// Returns an `AppError::NotFound` with an error message if the certification entry is not found.
    ///
    pub async fn certification_exists(
        &self,
        certification_id: i32,
    ) -> Result<Option<bool>, AppError> {
        let flag = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM certification WHERE id = $1")
            .bind(certification_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map_or_else(|_| None, |count| Some(count.0 > 0));

        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Certification not found".to_string(),
            }),
        }
    }
}
