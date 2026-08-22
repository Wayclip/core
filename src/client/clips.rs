use crate::{
    client::{WayclipClient, WayclipResponse},
    models::{
        clips::{
            games::ClipsGames,
            hosted::{
                ClipVisibility, ClipsNewMetadata, ClipsResponse, HostedClip,
                PatchClipsClipIdRequest,
            },
            tags::ClipsTag,
        },
        error::WayclipError,
        nutype::ClipNameSanitised,
        query::{FullQueryWeb, PaginatedResponseWeb},
    },
    settings::output::VideoFormat,
};
use itertools::Itertools;
use reqwest::{Method, multipart};
use serde_json::json;
use std::{mem::discriminant, path::PathBuf, sync::Arc};
use tokio::fs;

#[derive(Clone)]
pub struct ClipsHttpClient {
    client: WayclipClient,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatchClipField {
    Visibility(ClipVisibility),
    Name(String),
    Tags(Vec<ClipsTag>),
    Game(Option<ClipsGames>),
}

impl ClipsHttpClient {
    pub fn new(api_url: url::Url) -> Result<Self, WayclipError> {
        let client = WayclipClient::new(api_url)?;
        Ok(Self { client })
    }

    pub async fn query_me_clips(
        &mut self,
        query: FullQueryWeb,
    ) -> Result<PaginatedResponseWeb<HostedClip>, WayclipError> {
        let response: WayclipResponse<PaginatedResponseWeb<ClipsResponse>> = self
            .client
            .with_credentials()
            .await?
            .with_body(&query)
            .await?
            .send_call(Method::POST, "/users/me/clips/search")
            .await?;

        response.into_inner()
    }

    pub async fn delete(&mut self, clip_id: &str) -> Result<(), WayclipError> {
        self.client
            .with_credentials()
            .await?
            .send_call::<()>(Method::DELETE, &format!("/clips/{}", clip_id))
            .await?;
        Ok(())
    }

    pub async fn patch(
        &mut self,
        new_values: Vec<PatchClipField>,
        clip_id: &str,
    ) -> Result<(), WayclipError> {
        if new_values.is_empty() || !new_values.iter().map(discriminant).all_unique() {
            return Err(WayclipError::Validation(
                "Invalid new_values parameter".into(),
            ));
        }

        let mut name = None;
        let mut tags = None;
        let mut game = None;
        let mut visibility = None;

        for value in new_values {
            match value {
                PatchClipField::Name(n) => name = Some(n),
                PatchClipField::Tags(t) => tags = Some(t),
                PatchClipField::Visibility(v) => visibility = Some(v),
                PatchClipField::Game(g) => game = Some(g),
            }
        }

        let body = PatchClipsClipIdRequest {
            name: name.map(ClipNameSanitised::try_from).transpose()?,
            tags: tags
                .map(|ts| ts.into_iter().map(TryInto::try_into).collect())
                .transpose()?,
            detected_game: game,
            clip_visibility: visibility,
            comment_visibility: None,
        };

        let _response: WayclipResponse<ClipsResponse> = self
            .client
            .with_credentials()
            .await?
            .with_body(&body)
            .await?
            .send_call(Method::PATCH, &format!("/clips/{}", clip_id))
            .await?;

        Ok(())
    }

    // // make sure to save ID so we can reference it later
    // let now = chrono::Utc::now();
    // self.uploaded_at = Some(now);
    // self.uploaded_id = Some(result.clip_id);
    // self.update_local_metadata_file()?;

    pub async fn upload(
        &mut self,
        video_format: VideoFormat,
        video_path: &PathBuf,
        metadata: ClipsNewMetadata,
    ) -> Result<HostedClip, WayclipError> {
        // Get all the variables here that will be brought into enclosue
        let bytes = Arc::new(fs::read(video_path).await?);
        let file_name = metadata.name.clone().into_inner();
        let mime_string = video_format.get_mime_str().to_string();
        let json_string = json!(metadata).to_string();

        let multipart_builder = Arc::new(move || {
            // Construct file part
            let file_part = multipart::Part::bytes((*bytes).clone())
                .file_name(file_name.clone())
                .mime_str(&mime_string)
                .expect("valid mime type");

            // Construct metadata part
            let json_part = multipart::Part::text(json_string.clone())
                .mime_str("application/json")
                .expect("valid mime type");

            multipart::Form::new()
                .part("json", json_part)
                .part("file", file_part)
        });

        let response: WayclipResponse<HostedClip> = self
            .client
            .with_credentials()
            .await?
            .with_multipart(multipart_builder)
            .await
            .send_call(Method::POST, "clips")
            .await?;

        log::info!("Uploaded clip successfully");

        response.into_inner()
    }
}

// pub async fn search_hosted_clips(
//     api_url: &str,
//     query: FullQueryWeb,
// ) -> Result<PaginatedResponseWeb<HostedClip>, WayclipError> {
//     QueryClips::new(api_url, query)?.call().await
// }
// pub async fn query_hosted_clips(
//     api_url: String,
//     clip_identifier: &str,
//     regex: bool,
// ) -> Result<Vec<Self>, WayclipError> {
//     if regex {
//         let re = Regex::new(clip_identifier)?;
//         let all_clips = Self::fetch_all_hosted_clips(api_url).await?;

//         let matched = all_clips
//             .into_iter()
//             .filter(|clip| re.is_match(&clip.clip_id) || re.is_match(&clip.clip_name))
//             .collect();

//         Ok(matched)
//     } else {
//         let id_query = FullQueryWeb {
//             page_query: PageQuery {
//                 page_size: 100,
//                 page_num: 0,
//             },
//             vec_query: None,
//             order_query: None,
//             string_query: Some(StringQueryWeb {
//                 column: "clip_id".to_string(),
//                 equal: Some(clip_identifier.to_string()),
//                 like: None,
//                 start: None,
//                 end: None,
//             }),
//             range_query: None,
//         };

//         let mut results = Self::search_hosted_clips(api_url.clone(), id_query)
//             .await?
//             .current_page_items;

//         let name_query = FullQueryWeb {
//             page_query: PageQuery {
//                 page_size: 100,
//                 page_num: 0,
//             },
//             vec_query: None,
//             order_query: None,
//             string_query: Some(StringQueryWeb {
//                 column: "clip_name".to_string(),
//                 equal: Some(clip_identifier.to_string()),
//                 like: None,
//                 start: None,
//                 end: None,
//             }),
//             range_query: None,
//         };

//         let name_results = Self::search_hosted_clips(api_url, name_query)
//             .await?
//             .current_page_items;

//         for clip in name_results {
//             if !results.iter().any(|c| c.clip_id == clip.clip_id) {
//                 results.push(clip);
//             }
//         }

//         Ok(results)
//     }
// }

// pub async fn get_clip_by_identifier(
//     api_url: String,
//     clip_identifier: &str,
// ) -> Result<Option<Self>, WayclipError> {
//     let id_query = FullQueryWeb {
//         page_query: PageQuery {
//             page_size: 1,
//             page_num: 0,
//         },
//         vec_query: None,
//         order_query: None,
//         string_query: Some(StringQueryWeb {
//             column: "clip_id".to_string(),
//             equal: Some(clip_identifier.to_string()),
//             like: None,
//             start: None,
//             end: None,
//         }),
//         range_query: None,
//     };

//     let id_response = Self::search_hosted_clips(api_url.clone(), id_query).await?;
//     if let Some(clip) = id_response.current_page_items.into_iter().next() {
//         return Ok(Some(clip));
//     }

//     let name_query = FullQueryWeb {
//         page_query: PageQuery {
//             page_size: 1,
//             page_num: 0,
//         },
//         vec_query: None,
//         order_query: None,
//         string_query: Some(StringQueryWeb {
//             column: "clip_name".to_string(),
//             equal: Some(clip_identifier.to_string()),
//             like: None,
//             start: None,
//             end: None,
//         }),
//         range_query: None,
//     };

//     let name_response = Self::search_hosted_clips(api_url, name_query).await?;
//     Ok(name_response.current_page_items.into_iter().next())
// }

// pub async fn fetch_all_hosted_clips(api_url: String) -> Result<Vec<Self>, WayclipError> {
//     let auth_manager = Auth::new(api_url)?;
//     let mut url = auth_manager.api_endpoint.clone();
//     url.set_path("/users/me/clips/search");

//     let mut all_clips = Vec::new();
//     let mut page_num = 0;

//     loop {
//         let body = FullQueryWeb {
//             page_query: PageQuery {
//                 page_size: 100,
//                 page_num,
//             },
//             vec_query: None,
//             order_query: None,
//             string_query: None,
//             range_query: None,
//         };

//         let response: PaginatedResponseWeb<Self> = auth_manager
//             .send_auth_call::<_, FullQueryWeb>(Method::POST, url.clone(), None, Some(body))
//             .await?;

//         let num_pages = response.num_pages;
//         let fetched_count = response.current_page_items.len();

//         all_clips.extend(response.current_page_items);

//         page_num += 1;
//         if page_num >= num_pages || fetched_count == 0 {
//             break;
//         }
//     }

//     Ok(all_clips)
// }
