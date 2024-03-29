use crate::models::profile::{ProfileModel, PublicProfileModel};
use crate::response::error_handling::AppError;
use crate::{IdenoDBResult, IdenoPool};

#[derive(Clone)]
pub struct ProfileService {
    db_pool: IdenoPool,
}

impl ProfileService {
    pub fn new(db_pool: IdenoPool) -> Self {
        ProfileService { db_pool }
    }

    /// Asynchronously creates a profile for a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user for whom the profile is to be created.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the profile creation operation. If the profile is successfully created,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the profile creation operation.
    /// Logs an error using `tracing::error!` and returns `AppError::InternalError` if there is an error during the execution of the SQL query.
    ///
    pub async fn create_profile(&self, user_id: i32) -> Result<IdenoDBResult, AppError> {
        sqlx::query("INSERT INTO profiles (user_id) VALUES ($1)")
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error creating profile: {}", e);
                AppError::InternalError
            })
    }

    /// Asynchronously updates a user's profile in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose profile is to be updated.
    /// * `payload` - A `PublicProfileModel` containing the updated profile data.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the updated `ProfileModel` if the profile is successfully updated.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the profile update operation.
    ///
    pub async fn update_profile(
        &self,
        user_id: i32,
        payload: PublicProfileModel,
    ) -> Result<ProfileModel, AppError> {
        sqlx::query_as::<_, ProfileModel>(
            "UPDATE profiles SET first_name = $1, last_name = $2 , pronouns = $3, headline = $4, country = $5, city = $6, bio = $7 WHERE user_id = $8 RETURNING *",
        )
            .bind(payload.first_name)
            .bind(payload.last_name)
            .bind(payload.pronouns)
            .bind(payload.headline)
            .bind(payload.country)
            .bind(payload.city)
            .bind(payload.bio)
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| { AppError::InternalError })
    }

    /// Asynchronously retrieves a user's public profile from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose public profile is to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `PublicProfileModel` representing the user's public profile.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
    pub async fn get_public_profile(&self, user_id: i32) -> Result<PublicProfileModel, AppError> {
        sqlx::query_as::<_, PublicProfileModel>(
            "SELECT
                first_name,
                last_name,
                pronouns,
                headline,
                country,
                city,
                bio
            FROM profiles where user_id = ?",
        )
        .bind(user_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }
}
