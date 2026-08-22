use crate::models::{
    clips::{
        games::ClipsGames,
        tags::{ClipsClipTagRequest, ClipsClipTagResponse},
    },
    nutype::{
        BitrateKbpsSanitised, ClipCommentContentSanitised, ClipDurationSanitised,
        ClipNameSanitised, FpsSanitised, ResolutionSanitised,
    },
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::Display;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ClipsNewCommentNotificationMetadata {
    pub comment_id: String,
    pub clip_id: String,
    pub content: String,
    pub reply_to: Option<String>,
    pub commented_at: DateTime<FixedOffset>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ClipsNewCommentRequest {
    pub content: ClipCommentContentSanitised,
    pub parent_comment_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ClipsResponse {
    pub clip_id: String,
    pub clip_name: String,
    pub user_id: String,
    pub duration_s: i32,
    pub tags: Vec<ClipsClipTagResponse>,
    pub resolution: Option<String>,
    pub bitrate_kbps: Option<i32>,
    pub fps: Option<i32>,
    pub clip_status: ClipsStatusType,
    pub preview_status: ClipsStatusType,
    pub thumbnail_status: ClipsStatusType,
    pub file_size_mb: i32,
    pub uploaded_at: DateTime<FixedOffset>,
    pub detected_game: Option<ClipsGames>,
    pub locale: String,
    pub clip_visibility: ClipVisibility,
    pub comment_visibility: CommentVisibility,
    pub reactions: HashMap<String, i32>,
    pub reaction_count: i32,
    pub view_count: i32,
}

// makes naming more lenient
pub type HostedClip = ClipsResponse;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum ClipsStatusType {
    Unknown,
    Processing,
    Failed,
    Ready,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Display)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum ClipVisibility {
    Public,
    Private,
    Unlisted,
    FriendsOnly,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum CommentVisibility {
    Everyone,
    FriendsOnly,
    NoOne,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ClipsNewMetadata {
    pub name: ClipNameSanitised,
    pub duration_s: ClipDurationSanitised,
    pub tags: Vec<ClipsClipTagRequest>,
    pub resolution: Option<ResolutionSanitised>,
    pub bitrate_kbps: Option<BitrateKbpsSanitised>,
    pub fps: Option<FpsSanitised>,
    pub clip_visibility: ClipVisibility,
    pub detected_game: Option<ClipsGames>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PatchClipsClipIdRequest {
    pub clip_visibility: Option<ClipVisibility>,
    pub comment_visibility: Option<CommentVisibility>,
    pub name: Option<ClipNameSanitised>,
    pub tags: Option<Vec<ClipsClipTagRequest>>,
    pub detected_game: Option<Option<ClipsGames>>,
}
