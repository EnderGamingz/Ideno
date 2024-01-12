use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct CertificationModel {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub organization: String,
    pub issue_date: String,
    pub expiration_date: String,
    pub credential_id: String,
    pub credential_url: String,
    pub created_at: String,
    pub updated_at: String,
}
