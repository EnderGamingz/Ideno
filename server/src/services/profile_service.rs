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

    pub async fn create_profile(&self, user_id: i32) -> Result<IdenoDBResult, AppError> {
        sqlx::query("INSERT INTO profiles (user_id) VALUES ($1)")
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|e| {
                println!("Error creating profile: {}", e);
                AppError::InternalError
            })
    }

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
