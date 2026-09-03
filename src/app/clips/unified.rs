use crate::{
    app::clips::query::ClipsQuery,
    models::{
        clips::{
            games::ClipsGames,
            hosted::{ClipsStatusType, HostedClip},
            local::LocalClip,
            unified::{SelectedClip, UnifiedClip, UnifiedClipType},
        },
        error::WayclipError,
    },
    settings::{DEFAULT_TIME_FORMAT, UserSettings},
};
use chrono::{DateTime, Local};
use colored::ColoredString;
use std::fmt::Display;

/// A struct which contains information about a local clip.
/// This is used only to simplify logic and be able to share a single struct across CLI/GUI.
/// This struct also implmenets `Display`
#[derive(Debug, Clone, Default)]
pub struct LocalClipInfo {
    name: String,
    tags: Vec<ColoredString>,
    game: Option<ClipsGames>,
    created: DateTime<Local>,
    uploaded: Option<DateTime<Local>>,
    duration: f32,
    file_size_mb: u64,
    file_name: String,
}

/// A struct which contains information about a hosted clip.
/// This is used only to simplify logic and be able to share a single struct across CLI/GUI.
/// This struct also implmenets `Display`
#[derive(Debug, Clone, Default)]
pub struct HostedClipInfo {
    hosted_id: String,
    hosted_status: String,
    hosted_link: Option<String>,
    hosted_usage: i32,
}

/// A struct which contains information about any single clip
/// This is used only to simplify logic and be able to share a single struct across CLI/GUI.
/// This struct also implmenets `Display`
#[derive(Debug, Clone, Default)]
pub struct SelectedClipInfo {
    local: LocalClipInfo,
    hosted: HostedClipInfo,
}

impl Display for SelectedClipInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use the file_name to identify if the local was filled out
        if !self.local.file_name.is_empty() {
            writeln!(f, "Name: {}", self.local.name)?;

            if !self.local.tags.is_empty() {
                let tags_str = self
                    .local
                    .tags
                    .iter()
                    .map(|t| format!("[{}]", t))
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(f, "Tags: {}", tags_str)?;
            }

            if let Some(ref game) = self.local.game {
                writeln!(f, "Game: {:?}", game.display_name())?;
            }

            writeln!(
                f,
                "Created: {}",
                self.local.created.format(DEFAULT_TIME_FORMAT)
            )?;

            if let Some(u) = self.local.uploaded {
                writeln!(f, "Uploaded: {}", u.format(DEFAULT_TIME_FORMAT))?;
            }

            writeln!(f, "Duration: {:.2}s", self.local.duration)?;
            writeln!(f, "Local file size: {}MB", self.local.file_size_mb)?;
            writeln!(f, "File name: {}", self.local.file_name)?;
        }

        // Use the hosted_id to identify if the hosted was filled out
        if !self.hosted.hosted_id.is_empty() {
            writeln!(f, "Hosted ID: {}", self.hosted.hosted_id)?;

            if let Some(ref link) = self.hosted.hosted_link {
                writeln!(f, "Hosted link: {}", link)?;
            }

            writeln!(f, "Status: {}", self.hosted.hosted_status)?;
            writeln!(f, "Hosted storage usage: {}MB", self.hosted.hosted_usage)?;
        }

        Ok(())
    }
}

impl From<&HostedClip> for HostedClipInfo {
    fn from(hosted: &HostedClip) -> Self {
        let mut info = HostedClipInfo {
            hosted_id: hosted.clip_id.clone(),
            hosted_usage: hosted.file_size_mb,
            ..Default::default()
        };

        match (
            &hosted.preview_status,
            &hosted.clip_status,
            &hosted.thumbnail_status,
        ) {
            (ClipsStatusType::Ready, ClipsStatusType::Ready, ClipsStatusType::Ready) => {
                info.hosted_link = Some(
                    // TODO: fix hardocded wayclip.com
                    format!("https://wayclip.com/clips/{}", hosted.clip_id),
                );
                info.hosted_status = hosted.clip_visibility.to_string();
            }
            _ => {
                info.hosted_status = "Pending".to_string();
            }
        }

        info
    }
}

impl From<&LocalClip> for LocalClipInfo {
    fn from(local: &LocalClip) -> Self {
        let mut info = LocalClipInfo {
            name: local.name.clone(),
            tags: local
                .clone()
                .clip_tags
                .iter()
                .map(|t| t.get_colored_string())
                .collect::<Vec<_>>(),
            game: local.detected_game,
            created: DateTime::from(local.created_at),
            duration: local.file_duration_ms as f32 / 1000.0,
            file_size_mb: local.file_size_mb,
            file_name: local
                .video_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            ..Default::default()
        };

        if local.uploaded_id.is_some()
            && let Some(time) = local.uploaded_at
        {
            let local_time: DateTime<Local> = DateTime::from(time);
            info.uploaded = Some(local_time);
        }

        info
    }
}

impl From<&UnifiedClip> for SelectedClipInfo {
    fn from(unified_clip: &UnifiedClip) -> Self {
        let mut info = SelectedClipInfo::default();

        if let Some(ref local) = unified_clip.local {
            info.local = local.into();
        }

        if let Some(ref hosted) = unified_clip.hosted {
            info.hosted = hosted.into()
        }

        info
    }
}

impl From<&SelectedClip> for SelectedClipInfo {
    fn from(selected_clip: &SelectedClip) -> Self {
        let (local, hosted): (Option<&LocalClip>, Option<&HostedClip>) = match selected_clip {
            SelectedClip::Both(local, hosted) => (Some(local), Some(hosted)),
            SelectedClip::Local(local) => (Some(local), None),
            SelectedClip::Hosted(hosted) => (None, Some(hosted)),
        };

        let mut info = SelectedClipInfo::default();

        if let Some(local) = local {
            info.local = local.into();
        }

        if let Some(hosted) = hosted {
            info.hosted = hosted.into()
        }

        info
    }
}

impl UnifiedClip {
    /// The clips structs are a bit convoluted, but here is a general explanation:
    /// We have 2 core structs, LocalClip and HostedClip. One has metadata stored locally, whilst the
    /// other has to be fetched from the API.
    /// When user wants to interact with clips, we dont want to limit him to only one type of clip, so
    /// we have to mix them together. For that reason we have UnifiedClip.
    /// UnifiedClip has basic information needed for searching & sorting and then has optional `local`
    /// and `hosted` fields. The CLI/GUI will present user with all the found UnifiedClips and user will
    /// have to pick one of them.
    /// After user picks a UnifiedClip, we have to prompt him which specific version of the clip he
    /// wants to change. For that, we cast the UnifiedClip to a SelectedClip. Afterwards we can acquire
    /// MixedActions to see what actions are avaiable for this sepcific SelectedClip.
    /// TLDR; UnifiedClip -> Initial Clip Type for showing all clips. SelectedClip -> After user finds a
    /// clip and chooses which version to control (LocalClip, HostedClip, or both).
    pub async fn get_all_clips() -> Result<Vec<Self>, WayclipError> {
        // Initialise settings and a vec
        let settings = UserSettings::load()?;
        let mut unified_clips: Vec<Self> = Vec::new();

        // Collect all clips, local and hosted. If hosted errors, ignore and just give empty vec
        let mut hosted_clips: Vec<HostedClip> = match settings.api.enabled {
            true => ClipsQuery::get_all_hosted_clips().await.unwrap_or_default(),
            false => Vec::new(),
        };
        let local_clips = ClipsQuery::get_all_local_clips().await?;

        // Go through each local clip
        for mut local_clip in local_clips {
            // Local clips will act as the primary source of truth. Construct the unified clip
            // struct.
            let mut unified = UnifiedClip {
                name: local_clip.name.clone(),
                created_at: local_clip.created_at,
                local: Some(local_clip.clone()),
                hosted: None,
            };

            // If the clip is uploaded, we search the hosted_clips array and find the position it
            // was stored in
            if let Some(ref uploaded_id) = local_clip.uploaded_id {
                match hosted_clips.iter().position(|c| &c.clip_id == uploaded_id) {
                    Some(pos) => {
                        // If the clip was found, remove it from the hosted_clips array so we do not
                        // iterate through it once again and attach it to the unified struct
                        let hosted_clip = hosted_clips.remove(pos);
                        unified.hosted = Some(hosted_clip);
                    }
                    None => {
                        // If no clip was found with that ID, the metadata must be wrong. Hence, we
                        // log::warn and unbind the ID from local data.
                        // TODO: Reconsider permamnetly removing data
                        log::warn!(
                            "Clip {} is flagged as hosted, but was not found on remote under the ID {}",
                            local_clip.name,
                            uploaded_id
                        );

                        local_clip.uploaded_id = None;
                        local_clip.uploaded_at = None;
                        local_clip.update_metadata()?;
                    }
                }
            }

            unified_clips.push(unified);
        }

        // We iterate through the hosted_clips that we have left. Hopefully most of them were
        // removed once a link was found between a local and hosted clip.
        for hosted_clip in hosted_clips {
            if !unified_clips.iter().any(|c| {
                matches!(
                    &c.local,
                    Some(local) if local.uploaded_id == Some(hosted_clip.clip_id.clone())
                )
            }) {
                unified_clips.push(UnifiedClip {
                    name: hosted_clip.clip_name.clone(),
                    created_at: hosted_clip.uploaded_at.into(),
                    local: None,
                    hosted: Some(hosted_clip),
                })
            }
        }

        Ok(unified_clips)
    }
}

impl From<&UnifiedClip> for UnifiedClipType {
    fn from(value: &UnifiedClip) -> Self {
        match (&value.local, &value.hosted) {
            (Some(_), Some(_)) => UnifiedClipType::Both,
            (Some(_), None) => UnifiedClipType::LocalOnly,
            (None, Some(_)) => UnifiedClipType::HostedOnly,
            (None, None) => UnifiedClipType::None,
        }
    }
}
