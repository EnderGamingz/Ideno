use crate::models::certification::PublicCertificationModel;
use crate::models::contact_information::PublicContactInformationModel;
use crate::models::education::PublicEducationModel;
use crate::models::experience::PublicExperienceModel;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct ProfileModel {
    pub user_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub pronouns: Option<String>,
    pub headline: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub bio: Option<String>,
    pub created_at: String,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct PublicProfileModel {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub pronouns: Option<String>,
    pub headline: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub bio: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PublicProfileResponse {
    pub profile: PublicProfileModel,
    pub certification: Vec<PublicCertificationModel>,
    pub education: Vec<PublicEducationModel>,
    pub experience: Vec<PublicExperienceModel>,
    pub contact_information: Vec<PublicContactInformationModel>,
}
