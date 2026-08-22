use crate::{
    app::clips::ffmpeg::{Ffmpeg, PreviewGenerator},
    client::clips::ClipsHttpClient,
    models::{
        clips::{
            games::ClipsGames,
            hosted::{ClipVisibility, ClipsNewMetadata},
            local::LocalClip,
            tags::{ClipsClipTagColor, ClipsTag},
        },
        error::WayclipError,
    },
    settings::{
        UserSettings,
        output::VideoFormat,
        recording::{Bitrate, Fps, Resolution},
    },
};
use std::{os::unix::fs::MetadataExt, path::PathBuf};

impl LocalClip {
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        name: &str,
        video_format: VideoFormat,
        video_path: PathBuf,
        preview_path: PathBuf,
        metadata_path: PathBuf,
        detected_game: Option<ClipsGames>,
        duration_ms: Option<u64>,
        bitrate_kbps: Bitrate,
        resolution: Resolution,
        fps: Fps,
    ) -> Result<Self, WayclipError> {
        let now = chrono::Utc::now();
        let file_size_mb = video_path.metadata()?.size() / 1000000;
        let duration = match duration_ms {
            Some(ms) => ms,
            None => Ffmpeg::duration(&video_path)
                .await?
                .ok_or_else(|| WayclipError::NotFound("No duration found".into()))?
                .mseconds(),
        };

        let mut data = Self {
            name: name.to_string(),
            video_format,
            video_path,
            preview_path,
            metadata_path,
            created_at: now,
            file_duration_ms: duration,
            clip_end_ms: duration,
            file_size_mb,
            detected_game,
            modified_at: None,
            uploaded_at: None,
            uploaded_id: None,
            liked: false,
            clip_start_ms: 0,
            clip_tags: Vec::new(),
            bitrate_kbps,
            resolution,
            fps,
        };

        data.update_metadata()?;
        Ok(data)
    }

    /// Name
    pub fn set_name(&mut self, new_name: String) -> Result<(), WayclipError> {
        // By the way, this will only edit the name inside metadata.
        // For now, we keep the file name and clip name separate.
        // This is done mainly because when you upload a clip you probably want the name to look
        // nice, which is what is stored inside the metadata; however, we are not able to use same
        // name as a file name - it will look and parse horribly. We could theoretically parse it
        // and sanitise, but then name and file_name become very unsynced.
        self.name = new_name;
        self.update_metadata()
    }

    /// Game Type
    pub fn set_game_type(&mut self, new_game_type: Option<ClipsGames>) -> Result<(), WayclipError> {
        self.detected_game = new_game_type;
        self.update_metadata()
    }

    /// Tags
    pub fn set_local_tags(&mut self, tags: Vec<ClipsTag>) -> Result<(), WayclipError> {
        self.clip_tags = tags;
        self.update_metadata()
    }

    pub fn add_local_tag(
        &mut self,
        name: String,
        color: ClipsClipTagColor,
    ) -> Result<(), WayclipError> {
        self.clip_tags.push(ClipsTag { name, color });
        self.update_metadata()
    }

    pub fn remove_local_tag(&mut self, tag: &str) -> Result<(), WayclipError> {
        self.clip_tags.retain(|value| value.name != tag);
        self.update_metadata()
    }

    /// Like
    pub fn toggle_local_like(&mut self) -> Result<(), WayclipError> {
        self.liked = !self.liked;
        self.update_metadata()
    }

    /// Trim
    pub async fn trim(
        &mut self,
        preview_generator: &impl PreviewGenerator,
        target_path: &PathBuf,
        new_start_ms: u64,
        new_end_ms: u64,
    ) -> Result<(), WayclipError> {
        let settings = UserSettings::load()?;
        Ffmpeg::trim(&self.video_path, target_path, new_start_ms, new_end_ms).await?;

        let new_duration = Ffmpeg::duration(target_path)
            .await?
            .ok_or_else(|| WayclipError::Validation("Failed to get new duration".into()))?
            .mseconds();
        let new_file_size_mb = target_path.metadata()?.size() / 1_000_000;

        self.file_size_mb = new_file_size_mb;
        self.file_duration_ms = new_duration;
        self.clip_end_ms = new_duration;
        self.clip_start_ms = 0;

        // Since we are not overwritting an old file, we have to create a tottaly new one
        if target_path != &self.video_path {
            let name = target_path
                .clone()
                .file_stem()
                .ok_or_else(|| WayclipError::NotFound("No stem found".into()))?
                .to_string_lossy()
                .to_string();
            let metadata_path = settings
                .output
                .metadata_directory
                .0
                .join(format!("{name}.json"));
            let preview_path = settings
                .output
                .preview_directory
                .0
                .join(format!("{name}.preview.mkv"));

            self.metadata_path = metadata_path;
            self.preview_path = preview_path;
            self.name = name;
            self.video_path = target_path.to_owned();
        }

        Ffmpeg::generate_preview(preview_generator, &self.video_path, &self.preview_path)?;
        self.update_metadata()
    }

    /// Hosted
    pub fn remove_hosted_connection(&mut self) -> Result<(), WayclipError> {
        self.uploaded_at = None;
        self.uploaded_id = None;
        self.update_metadata()
    }

    pub async fn upload(&mut self, clip_visibility: ClipVisibility) -> Result<(), WayclipError> {
        let settings = UserSettings::load()?;
        let mut clips_client = ClipsHttpClient::new(settings.api.url)?;

        let mut new_clip_metadata: ClipsNewMetadata = self.clone().try_into()?;
        new_clip_metadata.clip_visibility = clip_visibility;

        let response = clips_client
            .upload(
                settings.output.video_format,
                &self.video_path,
                new_clip_metadata,
            )
            .await?;

        self.uploaded_at = Some(chrono::Utc::now());
        self.uploaded_id = Some(response.clip_id);
        self.update_metadata()
    }
}
