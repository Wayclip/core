use crate::models::clips::hosted::CommentVisibility;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

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

/// An enum responsobile for defining what search results will appear in a search engine
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum SearchIndexingVisibility {
    /// No links
    Hidden,
    /// Only users' clips
    Clips,
    /// Only users' profile
    Profile,
    /// Both clips and profile
    ProfileAndClips,
}

/// The notification settings that user will control, parsed as sea_orm::Value inside db
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct NotificationPreferences {
    /// Send notification when a new follow occurs
    pub user_follow: bool,
    /// Send notification when a new clip is uploaded
    pub clip_upload: bool,
    /// Send notification when a new reaction is added to comment/clip
    pub reaction_create: bool,
    /// Send notification when a new comment is added under a comment/clip
    pub comment_create: bool,
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self {
            user_follow: true,
            clip_upload: true,
            reaction_create: true,
            comment_create: true,
        }
    }
}

/// The available selection for the 'language' field for user
#[derive(Serialize, Deserialize, Display, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[allow(missing_docs)]
pub enum SupportedLanguages {
    #[serde(rename = "en-US")]
    #[strum(serialize = "en-US")]
    EnUs,
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
    /// The about me description of user
    pub about_me: Option<String>,
    /// The language identifier
    pub language: String,
    /// The geo-location of user
    pub location: String,
    /// The avatar's URL of user
    pub avatar_url: String,
    /// The storage limit that user
    pub storage_limit_id: Option<String>,
    /// Setting affecting the search index visibility
    pub search_indexing_visibility: SearchIndexingVisibility,
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
