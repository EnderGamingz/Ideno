use crate::models::contact_information::{
    AuthContactInformationModel, ContactInformationModel, PublicContactInformationModel,
};
use crate::response::error_handling::AppError;
use crate::{IdenoDBResult, IdenoPool};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddContactInformationPayload {
    pub contact_type: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateContactInformationPayload {
    pub contact_type: String,
    pub value: String,
}

#[derive(Clone)]
pub struct ContactInformationService {
    db_pool: IdenoPool,
}

impl ContactInformationService {
    pub fn new(db_pool: IdenoPool) -> Self {
        ContactInformationService { db_pool }
    }

    /// Asynchronously retrieves authenticated contact information associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the authenticated user whose contact information is to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `AuthContactInformationModel` instances representing the authenticated contact information associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
    pub async fn get_authenticated_contact_information(
        &self,
        user_id: i32,
    ) -> Result<Vec<AuthContactInformationModel>, AppError> {
        sqlx::query_as::<_, AuthContactInformationModel>(
            "SELECT
                id,
                type_field,
                value
                FROM contact_information
                WHERE user_id = $1
                ORDER BY created_at DESC",
        )
        .bind(&user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously retrieves public contact information associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose public contact information is to be retrieved.
    /// * `limit` - An optional limit on the number of contact information entries to retrieve. If not provided, retrieves all entries.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `PublicContactInformationModel` instances representing the public contact information associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
    pub async fn get_public_contact_information(
        &self,
        user_id: i32,
        limit: Option<i32>,
    ) -> Result<Vec<PublicContactInformationModel>, AppError> {
        let limit = limit.unwrap_or(-1);
        sqlx::query_as::<_, PublicContactInformationModel>(
            "SELECT
                type_field,
                value
                FROM contact_information
                WHERE user_id = $1
                ORDER BY created_at DESC
                LIMIT $2",
        )
        .bind(&user_id)
        .bind(&limit)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously checks if a user owns a specific contact information entry.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to check ownership.
    /// * `contact_information_id` - The ID of the contact information entry to check ownership against.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<bool>`. If the user owns the contact information entry, it returns `Some(true)`.
    /// If the user does not own the contact information entry, it returns `Some(false)`. If the contact information entry is not found, it returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    /// Returns an `AppError::NotFound` with an error message if the contact information entry is not found.
    ///
    pub async fn user_owns_contact_information(
        &self,
        user_id: i32,
        contact_information_id: i32,
    ) -> Result<Option<bool>, AppError> {
        let flag = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM contact_information WHERE id = $1 AND user_id = $2",
        )
        .bind(contact_information_id)
        .bind(user_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
        .map_or_else(|_| None, |count| Some(count.0 > 0));
        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Contact information not found".to_string(),
            }),
        }
    }

    /// Asynchronously deletes a contact information entry associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user who owns the contact information entry to be deleted.
    /// * `contact_information_id` - The ID of the contact information entry to be deleted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the delete operation. If the contact information entry is successfully deleted,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the delete operation.
    ///
    pub async fn delete_contact_information(
        &self,
        user_id: i32,
        contact_information_id: i32,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("DELETE FROM contact_information WHERE id = $1 AND user_id = $2")
            .bind(contact_information_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously updates a contact information entry associated with a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user who owns the contact information entry to be updated.
    /// * `contact_information_id` - The ID of the contact information entry to be updated.
    /// * `payload` - An `UpdateContactInformationPayload` containing the updated contact information details.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the update operation. If the contact information entry is successfully updated,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the update operation.
    ///
    pub async fn update_contact_information(
        &self,
        user_id: i32,
        contact_information_id: i32,
        payload: UpdateContactInformationPayload,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query(
            "UPDATE contact_information SET type_field = $1, value = $2 WHERE id = $3 AND user_id = $4",
        )
            .bind(payload.contact_type)
            .bind(payload.value)
            .bind(contact_information_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously creates a new contact information entry associated with a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the contact information is being created.
    /// * `payload` - An `AddContactInformationPayload` containing the details of the contact information to be created.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the ID of the newly created contact information entry if successful.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the insert operation.
    ///
    pub async fn create_contact_information(
        &self,
        user_id: i32,
        payload: AddContactInformationPayload,
    ) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("INSERT INTO contact_information (user_id, type_field, value) VALUES ($1, $2, $3) RETURNING id")
            .bind(user_id)
            .bind(payload.contact_type)
            .bind(payload.value)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|count| count.0)
    }

    /// Asynchronously checks if a contact information entry with the same type and value already exists for a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the existence of contact information is being checked.
    /// * `data` - A reference to an `AddContactInformationPayload` containing the type and value of the contact information to be checked.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether an existing contact information entry with the same type and value exists for the user. If such an entry exists, it returns `true`, otherwise `false`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the query.
    ///
    pub async fn get_existing_contact_information(
        &self,
        user_id: i32,
        data: &AddContactInformationPayload,
    ) -> Result<bool, AppError> {
        sqlx::query_as::<_, (i64,)>(
            "SELECT id FROM contact_information WHERE user_id = $1 AND type_field = $2 AND value = $3 LIMIT 1",
        )
            .bind(user_id)
            .bind(&data.contact_type)
            .bind(&data.value)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|count| count.len() > 0)
    }

    /// Asynchronously retrieves the count of contact information entries associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the count of contact information entries is being retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the count of contact information entries associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the query.
    ///
    pub async fn get_contact_information_count(&self, user_id: i32) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM contact_information WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|count| count.0)
    }

    /// Asynchronously retrieves all contact information entries associated with a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the contact information entries are being retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `ContactInformationModel` representing all contact information entries associated with the user.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the query.
    ///
    pub async fn get_all_contact_information(
        &self,
        user_id: i32,
    ) -> Result<Vec<ContactInformationModel>, AppError> {
        sqlx::query_as::<_, ContactInformationModel>(
            "SELECT * FROM contact_information WHERE user_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously deletes a contact information entry from the database foe administrative purposes.
    ///
    /// # Arguments
    ///
    /// * `contact_information_id` - The ID of the contact information entry to be deleted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the delete operation. If the operation is successful, it returns `Ok(())`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the delete operation.
    ///
    pub async fn admin_delete_contact_information(
        &self,
        contact_information_id: i32,
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM contact_information WHERE id = $1")
            .bind(contact_information_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|_| ())
    }

    /// Asynchronously checks if a contact information entry exists in the database.
    ///
    /// # Arguments
    ///
    /// * `contact_information_id` - The ID of the contact information entry to be checked.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an optional boolean value.
    /// If the contact information entry exists, it returns `Some(true)`, otherwise `Some(false)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the query.
    /// Returns an `AppError::NotFound` with an error message if the contact information entry is not found.
    ///
    pub async fn contact_info_exists(
        &self,
        contact_information_id: i32,
    ) -> Result<Option<bool>, AppError> {
        let flag =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM contact_information WHERE id = $1")
                .bind(contact_information_id)
                .fetch_one(&self.db_pool)
                .await
                .map_err(|_| AppError::InternalError)
                .map_or_else(|_| None, |count| Some(count.0 > 0));

        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Contact information not found".to_string(),
            }),
        }
    }
}
