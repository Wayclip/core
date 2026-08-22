use gstreamer::ClockTime;
use gstreamer_pbutils::Discoverer;
use rust_ffmpeg::{Codec, FFmpegBuilder};
use std::path::PathBuf;
use tokio::fs;
use url::Url;

use crate::models::error::WayclipError;

pub struct Ffmpeg;

impl Ffmpeg {
    pub async fn trim(
        source_path: &PathBuf,
        target_path: &PathBuf,
        new_start_ms: u64,
        new_end_ms: u64,
    ) -> Result<PathBuf, WayclipError> {
        let duration_ms = new_end_ms - new_start_ms;

        // Check if we are writting to same location or not.
        // If we are, then we have to make a temporary file
        let overwrite = target_path == source_path;
        let to_path = if overwrite {
            let parent = source_path
                .parent()
                .ok_or_else(|| WayclipError::NotFound("No parent directory".into()))?;
            let stem = source_path
                .file_stem()
                .ok_or_else(|| WayclipError::NotFound("No file stem".into()))?
                .to_string_lossy();
            let ext = source_path
                .extension()
                .ok_or_else(|| WayclipError::NotFound("No extension".into()))?
                .to_string_lossy();
            parent.join(format!("{}_trim_tmp.{}", stem, ext))
        } else {
            target_path.clone()
        };

        FFmpegBuilder::new()?
            .input(
                rust_ffmpeg::Input::new(source_path.clone())
                    .seek(rust_ffmpeg::Duration::from_millis(new_start_ms))
                    .duration(rust_ffmpeg::Duration::from_millis(duration_ms)),
            )
            .output(
                rust_ffmpeg::Output::new(to_path.clone())
                    .video_codec(Codec::copy())
                    .audio_codec(Codec::copy()),
            )
            .overwrite()
            .run()
            .await?;

        if overwrite {
            fs::rename(to_path, target_path).await?;
        }

        Ok(target_path.to_owned())
    }

    /// Although this is not an FFmpeg action, I have decided it is more suited in this module, as
    /// it works directly with video files, but is not an I/O operation.
    pub async fn duration(source_path: &PathBuf) -> Result<Option<ClockTime>, WayclipError> {
        gstreamer::init()?;

        let absolute = match source_path.is_absolute() {
            true => source_path,
            false => &std::fs::canonicalize(source_path)?,
        };

        let uri = Url::from_file_path(absolute).map_err(|_| {
            WayclipError::Validation(format!("Failed to convert to a URI: {:?}", absolute).into())
        })?;

        let discover = Discoverer::new(ClockTime::from_seconds(10))?;
        let info = discover.discover_uri(uri.as_ref())?;

        Ok(info.duration())
    }

    /// Although this is not an FFmpeg action, I have decided it is more suited in this module, as
    /// it works directly with video files, but is not an I/O operation.
    pub fn generate_preview(
        generator: &impl PreviewGenerator,
        video_path: &PathBuf,
        preview_path: &PathBuf,
    ) -> Result<(), WayclipError> {
        generator.generate_preview(video_path, preview_path)
    }
}

/// To avoid dependency deadlock, we create a trait.
/// This trait allows us to pass in the generator as an argument.
/// This means that cli will be able to import both daemon and core crate, then pass in the
/// RemuxHandler (which not implements this trait) as a generator.
/// This simplifies the dependency tree to daemon -> core
pub trait PreviewGenerator: Send + Sync {
    fn generate_preview(
        &self,
        video_path: &PathBuf,
        preview_path: &PathBuf,
    ) -> Result<(), WayclipError>;
}
