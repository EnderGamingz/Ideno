use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct CertificationModel {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub organization: String,
    pub issue_date: Option<String>,
    pub expiration_date: Option<String>,
    pub credential_id: Option<String>,
    pub credential_url: Option<String>,
    pub created_at: String,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct PublicCertificationModel {
    pub name: String,
    pub organization: String,
    pub issue_date: Option<String>,
    pub expiration_date: Option<String>,
    pub credential_id: Option<String>,
    pub credential_url: Option<String>,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct AuthCertificationModel {
    pub id: i32,
    pub name: String,
    pub organization: String,
    pub issue_date: Option<String>,
    pub expiration_date: Option<String>,
    pub credential_id: Option<String>,
    pub credential_url: Option<String>,
}