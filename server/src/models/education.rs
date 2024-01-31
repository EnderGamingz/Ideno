use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct EducationModel {
    pub id: i32,
    pub user_id: i32,
    pub school: String,
    pub degree: Option<String>,
    pub field: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub created_at: String,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct PublicEducationModel {
    pub school: String,
    pub degree: Option<String>,
    pub field: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct AuthEducationModel {
    pub id: i32,
    pub school: String,
    pub degree: Option<String>,
    pub field: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}