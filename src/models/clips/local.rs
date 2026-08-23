use crate::{
    models::{
        clips::{
            games::ClipsGames,
            hosted::{ClipVisibility, ClipsNewMetadata},
            tags::ClipsTag,
        },
        error::WayclipError,
        nutype::{
            BitrateKbpsSanitised, ClipDurationSanitised, ClipNameSanitised, FpsSanitised,
            ResolutionSanitised,
        },
    },
    settings::{
        output::VideoFormat,
        recording::{Bitrate, Fps, Resolution},
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Method with which trimming clips is done
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveMethod {
    /// Replace current file
    Replace,
    /// Copy & make new
    Copy,
}

/// The data stored in the metadata about a local clip
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalClip {
    /// The name of clip
    pub name: String,
    /// The path for the video file
    pub video_path: PathBuf,
    /// The path for the metadata file
    pub metadata_path: PathBuf,
    /// The path for the preview file
    pub preview_path: PathBuf,
    /// Time created at
    pub created_at: DateTime<Utc>,
    /// Time modified at
    pub modified_at: Option<DateTime<Utc>>,
    /// Time uploaded at (or None if still local)
    pub uploaded_at: Option<DateTime<Utc>>,
    /// Uploaded ID (or None if still local)
    pub uploaded_id: Option<String>,
    /// The tags of a clip
    pub clip_tags: Vec<ClipsTag>,
    /// If clip is liked
    pub liked: bool,
    /// Start time of clip (kinda redundant rn)
    pub clip_start_ms: u64,
    /// End time of clip (kinda redundant rn)
    pub clip_end_ms: u64,
    /// Duration of clip
    pub file_duration_ms: u64,
    /// The file size of clip
    pub file_size_mb: u64,
    /// Format clip was recorded in (mkv/mp4)
    pub video_format: VideoFormat,
    /// Bitrate clip was recorded in
    pub bitrate_kbps: Bitrate,
    /// Resolution clip was recorded in
    pub resolution: Resolution,
    /// FPs clip was recorded in
    pub fps: Fps,
    /// The GameType assosciated with this clip
    pub detected_game: Option<ClipsGames>,
}

impl TryFrom<LocalClip> for ClipsNewMetadata {
    type Error = WayclipError;

    fn try_from(value: LocalClip) -> Result<Self, Self::Error> {
        Ok(ClipsNewMetadata {
            name: ClipNameSanitised::try_from(value.name)?,
            duration_s: ClipDurationSanitised::try_from((value.file_duration_ms / 1000) as i32)?,
            bitrate_kbps: Some(BitrateKbpsSanitised::try_from(value.bitrate_kbps.0 as i32)?),
            tags: value
                .clip_tags
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            clip_visibility: ClipVisibility::Public,
            detected_game: value.detected_game,
            resolution: Some(ResolutionSanitised::try_from(value.resolution.to_string())?),
            fps: Some(FpsSanitised::try_from(value.fps.0 as i32)?),
        })
    }
}
