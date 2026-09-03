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

/// The request to leave a new comment
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ClipsNewCommentRequest {
    /// Content of comment
    pub content: ClipCommentContentSanitised,
    /// Parent ID of under what we are replying to
    pub parent_comment_id: Option<String>,
}

/// Data of a clip
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ClipsResponse {
    /// Id of clip
    pub clip_id: String,
    /// Name of clip
    pub clip_name: String,
    /// Id of clip's owner
    pub user_id: String,
    /// Duration of clip in seconds
    pub duration_s: i32,
    /// Tags linked to clip
    pub tags: Vec<ClipsClipTagResponse>,
    /// Original resolution of clip
    pub resolution: Option<String>,
    /// Original bitrate of clip, in kbps
    pub bitrate_kbps: Option<i32>,
    /// Original fps of clip
    pub fps: Option<i32>,
    /// The upload status of clip
    pub clip_status: ClipsStatusType,
    /// The upload status of preview
    pub preview_status: ClipsStatusType,
    /// The upload status of thumbnail
    pub thumbnail_status: ClipsStatusType,
    /// The original file size
    pub file_size_mb: i32,
    /// Time uploaded at
    pub uploaded_at: DateTime<FixedOffset>,
    /// Game assosciated with clip
    pub detected_game: Option<ClipsGames>,
    /// The location of the clip of user's choice
    pub location: String,
    /// Visibility of clip
    pub clip_visibility: ClipVisibility,
    /// Visibility/Access of comments
    pub comment_visibility: CommentVisibility,
    /// All reactions
    pub reactions: HashMap<String, i32>,
    /// Reaction count
    pub reaction_count: i32,
    /// View count
    pub view_count: i32,
}

/// An alias for ClipsResponse, to be used inside of APP
pub type HostedClip = ClipsResponse;

/// The uploading status of a clip/preview/thumbnail
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum ClipsStatusType {
    /// We dont know
    Unknown,
    /// Still processing
    Processing,
    /// Failed upload
    Failed,
    /// Fully uploaded
    Ready,
}

/// Visibility of clip - who can see the clip
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Display)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum ClipVisibility {
    /// Public, everyone
    Public,
    /// Only owner
    Private,
    /// Everyone with the link
    Unlisted,
    /// Only mutual subscribers
    FriendsOnly,
}

/// The comment visibility - who can comment under a clip
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum CommentVisibility {
    /// Everyone can comment
    Everyone,
    /// Only mutual subscribers can comment
    FriendsOnly,
    /// No one can comment
    NoOne,
}

/// Metadata to be put into the multipart when uploading a clip
/// Every field here has to be sanitised via `nutype`
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ClipsNewMetadata {
    /// The name of clip
    pub name: ClipNameSanitised,
    /// The duration
    pub duration_s: ClipDurationSanitised,
    /// The tags
    pub tags: Vec<ClipsClipTagRequest>,
    /// The original resolution
    pub resolution: Option<ResolutionSanitised>,
    /// The original bitrate
    pub bitrate_kbps: Option<BitrateKbpsSanitised>,
    /// The original fps
    pub fps: Option<FpsSanitised>,
    /// The visibility of clip
    pub clip_visibility: ClipVisibility,
    /// The game linked to clip
    pub detected_game: Option<ClipsGames>,
}

/// The request to be sent when patching a clip
/// Each field is `Option<T>` to make sure you can edit any single field at a time
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PatchClipsClipIdRequest {
    /// Visibility of clip
    pub clip_visibility: Option<ClipVisibility>,
    /// Visibility of comments
    pub comment_visibility: Option<CommentVisibility>,
    /// Name of clip
    pub name: Option<ClipNameSanitised>,
    /// Tags of clip
    pub tags: Option<Vec<ClipsClipTagRequest>>,
    /// Game Type of clip
    pub detected_game: Option<Option<ClipsGames>>,
}
