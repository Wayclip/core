use crate::models::clips::hosted::CommentVisibility;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct UsersLimitResponse {
    pub limit_id: String,
    pub limit_name: String,
    pub limit_mb: i32,
    pub used_mb: i64,
    pub limit_description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct UsersResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub locale: String,
    pub avatar_url: Option<String>,
    pub storage_limit_id: Option<String>,
    pub default_comment_visibility: CommentVisibility,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub banned_at: Option<DateTime<FixedOffset>>,
    pub verified_at: Option<DateTime<FixedOffset>>,
    pub deleted_at: Option<DateTime<FixedOffset>>,
}
