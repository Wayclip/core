use crate::{
    client::clips::ClipsHttpClient,
    models::{
        clips::{
            hosted::HostedClip,
            local::LocalClip,
            unified::{SelectedClip, SelectedClipType},
        },
        error::WayclipError,
        query::{FullQueryWeb, PageQuery, StringQueryWeb},
    },
    settings::UserSettings,
};
use itertools::Itertools;
use regex::Regex;
use std::ffi::OsStr;

/// An empty struct acting as a wrapper, containing query-related methods
pub struct ClipsQuery;

impl ClipsQuery {
    /// Use the HTTP client and web query to loop until we get all clips that user has hosted
    pub async fn get_all_hosted_clips() -> Result<Vec<HostedClip>, WayclipError> {
        // Load in the settings and construct the HTTP client
        let settings = UserSettings::load()?;
        let mut clips_client = ClipsHttpClient::new(settings.api.url)?;

        let mut all_clips = Vec::new();
        let mut page_num = 0;

        loop {
            // Fetch 1 page at a time, with 100 clips each
            let body = FullQueryWeb {
                page_query: PageQuery {
                    page_size: 100,
                    page_num,
                },
                vec_query: None,
                order_query: None,
                string_query: None,
                range_query: None,
            };

            let response = clips_client.query_me_clips(body).await?;
            let num_pages = response.num_pages;
            let fetched_count = response.current_page_items.len();

            all_clips.extend(response.current_page_items);

            // Keep going until we run out of pages or nothing is returned
            page_num += 1;
            if page_num >= num_pages || fetched_count == 0 {
                break;
            }
        }

        Ok(all_clips)
    }

    /// Use the HTTP client and web query to search for a specific clip on the remote
    /// Clip identifier could be either file name, clip name, or even a regex expression.
    pub async fn query_hosted_clips(
        clip_identifier: &str,
        regex: bool,
    ) -> Result<Vec<HostedClip>, WayclipError> {
        let settings = UserSettings::load()?;
        let mut clips_client = ClipsHttpClient::new(settings.api.url)?;

        // Query builder so we can change the column easily
        let build_query = |column: String| FullQueryWeb {
            page_query: PageQuery {
                page_size: 100,
                page_num: 0,
            },
            vec_query: None,
            order_query: None,
            string_query: Some(StringQueryWeb {
                column,
                like: None,
                equal: if !regex {
                    Some(clip_identifier.to_string())
                } else {
                    None
                },
                start: None,
                end: None,
                regex: if regex {
                    Some(clip_identifier.to_string())
                } else {
                    None
                },
            }),
            range_query: None,
        };

        // `clip_id` has to match what is specified in DB
        let id_query = build_query("clip_id".to_string());
        let mut results = clips_client
            .query_me_clips(id_query)
            .await?
            .current_page_items;

        // `clip_name` has to match what is specified in DB
        let name_query = build_query("clip_name".to_string());
        let name_results = clips_client
            .query_me_clips(name_query)
            .await?
            .current_page_items;

        // Join & dedup
        for clip in name_results {
            if !results.iter().any(|c| c.clip_id == clip.clip_id) {
                results.push(clip);
            }
        }

        Ok(results)
    }

    /// We made an iterator so we dont have to repeat ourselves so much when searching from local
    /// clips
    fn scan_clips(
        settings: &UserSettings,
    ) -> Result<impl Iterator<Item = (LocalClip, String)> + '_, WayclipError> {
        let entries = settings.output.metadata_directory.0.read_dir()?;

        let iterator = entries.flatten().filter_map(|entry| {
            let path = entry.path();

            // Now, we only process json files
            if !path.is_file() || path.extension().and_then(OsStr::to_str) != Some("json") {
                return None;
            }

            match LocalClip::from_metadata(&path) {
                Ok(clip) => {
                    let file_name = clip
                        .video_path
                        .file_name()
                        .and_then(OsStr::to_str)
                        .unwrap_or_default()
                        .to_string();

                    Some((clip, file_name))
                }
                Err(e) => {
                    log::warn!("Could not load clip metadata at {}: {e}", path.display());
                    None
                }
            }
        });

        Ok(iterator)
    }

    /// Method just to return every single local clip stored
    pub async fn get_all_local_clips() -> Result<Vec<LocalClip>, WayclipError> {
        let settings = UserSettings::load()?;

        // Use the already existing iterator
        let clips = Self::scan_clips(&settings)?.map(|(clip, _)| clip).collect();

        Ok(clips)
    }

    /// This query will be used to find all clips stored locally based on a clip_identifier.
    /// Clip identifier could be either file name, clip name, or even a regex expression.
    pub async fn query_local_clips(
        clip_identifier: &str,
        regex: bool,
    ) -> Result<Vec<LocalClip>, WayclipError> {
        let settings = UserSettings::load()?;

        // See if we need to do regex work
        let compiled_regex = match regex {
            true => Some(Regex::new(clip_identifier)?),
            false => None,
        };

        // Use the already existing iterator
        let clips = Self::scan_clips(&settings)?
            .filter(|(clip, file_name)| match &compiled_regex {
                Some(regex) => regex.is_match(&clip.name) || regex.is_match(file_name),
                None => clip.name == clip_identifier || file_name == clip_identifier,
            })
            .map(|(clip, _)| clip)
            .collect();

        Ok(clips)
    }

    /// Local method to find a clip by its identifier
    /// Clip identifier could be either file name or a clip name
    pub async fn get_local_clip(clip_identifier: &str) -> Result<LocalClip, WayclipError> {
        let settings = UserSettings::load()?;

        // Use the already existing iterator
        let clip = Self::scan_clips(&settings)?
            // Use find so we can get 1 result
            .find(|(clip, file_name)| {
                clip.name == clip_identifier
                    || file_name == clip_identifier
                    || clip.uploaded_id.as_deref() == Some(clip_identifier)
            })
            .map(|(clip, _)| clip)
            .ok_or_else(|| WayclipError::NotFound("No clip was found".into()))?;

        Ok(clip)
    }

    /// Gets all local clips, merges their tags and removes duplicates
    /// This method is used as a 'pseduo-autocomplete' when filling out tags.
    /// All your tags will be a suggestion that you can just TAB into
    pub async fn get_all_tags() -> Result<Vec<String>, WayclipError> {
        Ok(Self::get_all_local_clips()
            .await?
            .clone()
            .into_iter()
            .flat_map(|c| c.clip_tags.into_iter().map(|t| t.name))
            .unique()
            .collect::<Vec<_>>())
    }

    /// Gets all local clips, collects their names and removes duplicates
    pub async fn get_all_names() -> Result<Vec<String>, WayclipError> {
        Ok(Self::get_all_local_clips()
            .await?
            .clone()
            .into_iter()
            .map(|c| c.name)
            .unique()
            .collect::<Vec<_>>())
    }

    /// Handles searching both local & hosted
    /// Clip identifier could be either file name, clip name, or even a regex expression.
    /// `clip_type` has to be provided to know which sector to look for specifically
    /// TODO: Can be merged with `get_from_all_clips`
    pub async fn query_all_clips(
        clip_identifier: &str,
        regex: bool,
        clip_type: SelectedClipType,
    ) -> Result<Vec<SelectedClip>, WayclipError> {
        match clip_type {
            SelectedClipType::Local => {
                let local = Self::query_local_clips(clip_identifier, regex).await?;
                Ok(local.into_iter().map(SelectedClip::Local).collect())
            }
            SelectedClipType::Hosted => {
                let hosted = Self::query_hosted_clips(clip_identifier, regex).await?;
                Ok(hosted.into_iter().map(SelectedClip::Hosted).collect())
            }
            _ => Err(WayclipError::Validation("No such option".into())),
        }
    }

    /// Method to get a single clip from both local & hosted
    /// Clip identifier could be either file name, clip name, or even a regex expression.
    pub async fn get_from_all_clips(
        clip_identifier: &str,
        clip_type: SelectedClipType,
    ) -> Result<SelectedClip, WayclipError> {
        let all_clips = Self::query_all_clips(clip_identifier, false, clip_type).await?;
        // Cheat and aquire first clip
        let clip = all_clips
            .first()
            .ok_or_else(|| WayclipError::NotFound("No clip was found".into()))?;

        Ok(clip.to_owned())
    }
}
