use crate::models::error::WayclipError;
use gstreamer::ClockTime;
use gstreamer_pbutils::Discoverer;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::{fs, process::Command};
use url::Url;

/// An empty struct, created to make sure all the FFmpeg actions stay in one place.
pub struct Ffmpeg;

impl Ffmpeg {
    /// Trims the clip via the specified starting and ending positions.
    /// `Replace/Copy` logic is detected by comparing `source_path` and `target_path`.
    pub async fn trim(
        source_path: &PathBuf,
        target_path: &PathBuf,
        new_start_ms: u64,
        new_end_ms: u64,
    ) -> Result<PathBuf, WayclipError> {
        if new_start_ms >= new_end_ms {
            return Err(WayclipError::Validation(
                "Start timestamp must be less than end timestamp".into(),
            ));
        }

        let duration_ms = new_end_ms - new_start_ms;

        // Check if we are writing to same location or not.
        // If we are, then we have to make a temporary file
        let source_parent_raw = source_path
            .parent()
            .ok_or_else(|| WayclipError::NotFound("No source parent directory".into()))?;

        let target_parent_raw = target_path
            .parent()
            .ok_or_else(|| WayclipError::NotFound("No target parent directory".into()))?;

        let source_parent = tokio::fs::canonicalize(source_parent_raw)
            .await
            .unwrap_or_else(|_| source_parent_raw.to_path_buf());

        let target_parent = tokio::fs::canonicalize(target_parent_raw)
            .await
            .unwrap_or_else(|_| target_parent_raw.to_path_buf());

        let overwrite =
            source_parent == target_parent && source_path.file_name() == target_path.file_name();

        let to_path = if overwrite {
            let stem = source_path
                .file_stem()
                .ok_or_else(|| WayclipError::NotFound("No file stem".into()))?
                .to_string_lossy();
            let ext = source_path
                .extension()
                .ok_or_else(|| WayclipError::NotFound("No extension".into()))?
                .to_string_lossy();
            source_parent.join(format!("{}_trim_tmp.{}", stem, ext))
        } else {
            target_path.clone()
        };

        let start_sec = format!("{:.3}", new_start_ms as f64 / 1000.0);
        let duration_sec = format!("{:.3}", duration_ms as f64 / 1000.0);

        let mut cmd = Command::new("ffmpeg");
        cmd.kill_on_drop(true);
        cmd.args(["-y", "-ss", &start_sec, "-i"])
            .arg(source_path)
            .args(["-t", &duration_sec, "-c", "copy"])
            .arg(&to_path);

        let output = tokio::time::timeout(Duration::from_secs(60), cmd.output())
            .await
            .map_err(|_| WayclipError::CLI("FFmpeg trim timed out".into()))?
            .map_err(|e| WayclipError::CLI(format!("Failed to run FFmpeg: {e}").into()))?;

        if !output.status.success() {
            return Err(WayclipError::CLI(
                format!(
                    "FFmpeg trim failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
                .into(),
            ));
        }

        if overwrite {
            fs::rename(to_path, target_path).await?;
        }

        Ok(target_path.to_owned())
    }

    /// This method allows us to get the duration of a video using `gstreamer`
    /// P.S. Although this is not an FFmpeg action, I have decided it is more suited in this module, as
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

    /// This method allows us to generate a preview. This method also requires the generator itself
    /// to be passed in. So far, the only generator accepted is `RemuxHandler` from the
    /// `wayclip-daemon` crate.
    /// Although this is not an FFmpeg action, I have decided it is more suited in this module, as
    /// it works directly with video files, but is not an I/O operation.
    pub fn generate_preview(
        generator: &impl PreviewGenerator,
        video_path: &Path,
        preview_path: &Path,
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
    /// The method itself that is then implemented in the `wayclip-daemon` crate
    fn generate_preview(&self, video_path: &Path, preview_path: &Path) -> Result<(), WayclipError>;
}
