use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::auth::check_user::check_user;
use crate::models::profile::ProfileModel;
use crate::{AppError, AppState};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileUpdatePayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub pronouns: Option<String>,
    pub headline: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub bio: Option<String>,
}

pub async fn update_profile(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<ProfileUpdatePayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let pronouns_map = [("he", "he/him"), ("she", "she/her"), ("they", "they/them")];
    let personal_pronouns = if let Some(user_pronouns) = &payload.pronouns {
        let matched_pronouns = pronouns_map
            .iter()
            .find(|(key, _)| key == user_pronouns)
            .map(|(_, value)| Some(value.to_string()));

        matched_pronouns.unwrap_or_else(|| Some(user_pronouns.clone()))
    } else {
        None
    };

    let profile = sqlx::query_as::<_, ProfileModel>(
        "UPDATE profiles SET first_name = $1, last_name = $2 , pronouns = $3, headline = $4, country = $5, city = $6, bio = $7 WHERE user_id = $8 RETURNING *",
    )
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(personal_pronouns)
    .bind(&payload.headline)
    .bind(&payload.country)
    .bind(&payload.city)
    .bind(&payload.bio)
    .bind(user.id)
    .fetch_one(&*state.db)
    .await
    .map_err(|err| {
        println!("Error: {:?}", err);
        AppError::InternalError
    })?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&profile).unwrap()))
        .unwrap())
}
