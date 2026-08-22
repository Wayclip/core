use crate::{
    app::clips::query::ClipsQuery,
    models::{clips::local::LocalClip, error::WayclipError},
    settings::{UserSettings, output::Prune},
};
use std::{fs, path::PathBuf};

impl LocalClip {
    /// Create an instance of LocalClip from the metadata path
    pub fn from_metadata(metadata_path: &PathBuf) -> Result<Self, WayclipError> {
        let data = fs::read_to_string(metadata_path)?;
        let clip: Self = serde_json::from_str(&data)?;
        Ok(clip)
    }

    /// When a value is updated, we need to update the corresponding metadata file
    pub fn update_metadata(&mut self) -> Result<(), WayclipError> {
        // Change the modified at timestamp & convert to a prettified string
        self.modified_at = Some(chrono::Utc::now());
        let pretty_json = serde_json::to_string_pretty(&self)?;

        // Ensure all parent folders exist leading up to the metadata file itself
        if let Some(parent) = &self.metadata_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.metadata_path, pretty_json)?;

        Ok(())
    }

    /// A `new_name` can be provided, the clip will then be renamed and metadata updated
    pub fn rename(&mut self, new_name: &str) -> Result<(), WayclipError> {
        // We clone the path, and use it directly to change the file name
        let mut path = self.video_path.clone();
        path.set_file_name(format!(
            "{}.{}",
            new_name,
            self.video_format.get_extension()
        ));

        // Then write
        fs::rename(&self.video_path, path)?;

        // And finally update the clip metadata
        self.name = new_name.to_string();
        self.update_metadata()
    }

    /// Removes video, preview and metadata directly
    pub fn delete(&mut self) -> Result<(), WayclipError> {
        fs::remove_file(self.video_path.clone())?;
        fs::remove_file(self.preview_path.clone())?;
        fs::remove_file(self.metadata_path.clone())?;

        Ok(())
    }

    /// Prune will serve as a function to remove old clips, by following rules defined in user
    /// settings under `output.limit.prune`
    pub async fn prune() -> Result<(), WayclipError> {
        // Load settings and fetch all clips
        let settings = UserSettings::load()?;
        let mut all_clips = ClipsQuery::get_all_local_clips().await?;

        match settings.output.limit.prune {
            Prune::Age(age) => {
                let duration = age.get_duration();

                // Iterate through every clip, and if its older than the specified age - delete.
                for mut clip in all_clips {
                    let clip_age = chrono::Utc::now().signed_duration_since(clip.created_at);
                    if clip_age.num_milliseconds() > duration.as_millis() as i64 {
                        clip.delete()?;
                    }
                }
            }
            Prune::Clips(clip_num) => {
                // Sort so that new clips will go first in the array. This allows us to pop the back
                // (the oldest clips)
                all_clips.sort_by_key(|b| std::cmp::Reverse(b.created_at));

                // Keep removing and popping clips until we are under the limit
                while all_clips.len() > clip_num && clip_num != 0 {
                    let mut last = all_clips
                        .pop()
                        .ok_or_else(|| WayclipError::NotFound("No last clip".into()))?;
                    last.delete()?;
                }
            }
            Prune::SizeMb(size_mb) => {
                all_clips.sort_by_key(|b| std::cmp::Reverse(b.created_at));
                // Iterate and use `sum` method to sum up all the file sizes together.
                // We are not able to read directly from directory, as that would include previews,
                // metadata, etc..
                let mut total_size = all_clips
                    .clone()
                    .iter()
                    .map(|c| c.file_size_mb)
                    .sum::<u64>();

                // Keep removing and popping clips until we are under the limit
                while total_size > size_mb {
                    let mut last = all_clips
                        .pop()
                        .ok_or_else(|| WayclipError::NotFound("No last clip".into()))?;
                    total_size -= last.file_size_mb;
                    last.delete()?;
                }
            }
            Prune::DurationS(dur) => {
                // ITerate through evert clip and check if its duration is larger, if true - remove.
                for mut clip in all_clips {
                    if clip.file_duration_ms > dur * 1000 {
                        clip.delete()?;
                    }
                }
            }
            Prune::Disabled => (),
        }

        Ok(())
    }
}
