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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveMethod {
    Replace,
    Copy,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalClip {
    pub name: String,
    pub video_path: PathBuf,
    pub metadata_path: PathBuf,
    pub preview_path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub uploaded_at: Option<DateTime<Utc>>,
    pub uploaded_id: Option<String>,
    pub clip_tags: Vec<ClipsTag>,
    pub liked: bool,
    pub clip_start_ms: u64,
    pub clip_end_ms: u64,
    pub file_duration_ms: u64,
    pub file_size_mb: u64,
    pub video_format: VideoFormat,
    pub bitrate_kbps: Bitrate,
    pub resolution: Resolution,
    pub fps: Fps,
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
