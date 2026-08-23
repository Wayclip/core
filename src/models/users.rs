use crate::models::clips::hosted::CommentVisibility;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// The storage limit of a user. Response recieved from `/users/me/limit`
/// Traits locked behind the `openapi` feature
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct UsersLimitResponse {
    /// The storage limit ID
    pub limit_id: String,
    /// The storage limit name
    pub limit_name: String,
    /// The MB limit set on the storage limit
    pub limit_mb: i32,
    /// The storage limit description
    pub limit_description: Option<String>,
    /// The storage user has already used up
    pub used_mb: i64,
}

/// The data of a user. Response recieved from `/users/me`
/// Traits locked behind the `openapi` feature
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct UsersResponse {
    /// The ID of user
    pub user_id: String,
    /// The username of user
    pub username: String,
    /// The email of user
    pub email: String,
    /// The locale, e.g. `en-US`, of user
    pub locale: String,
    /// The avatar's URL of user
    pub avatar_url: Option<String>,
    /// The storage limit that user
    pub storage_limit_id: Option<String>,
    /// The comment visibility to be used by default
    pub default_comment_visibility: CommentVisibility,
    /// Time of creation of user
    pub created_at: DateTime<FixedOffset>,
    /// Time user was last updated
    pub updated_at: DateTime<FixedOffset>,
    /// Time user was banned
    pub banned_at: Option<DateTime<FixedOffset>>,
    /// Time user was verified
    pub verified_at: Option<DateTime<FixedOffset>>,
    /// Time user was deleted
    pub deleted_at: Option<DateTime<FixedOffset>>,
}
