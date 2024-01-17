use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct ExperienceModel {
    pub id: i32,
    pub user_id: i32,
    pub company: String,
    pub title: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub exp_type: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct PublicExperienceModel {
    pub company: String,
    pub title: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub exp_type: Option<String>,
    pub description: Option<String>,
}

