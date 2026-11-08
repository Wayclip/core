use crate::{
    client::{WayclipClient, WayclipResponse},
    models::{
        clips::{
            games::Game,
            hosted::{
                ClipVisibility, ClipsNewMetadata, ClipsResponse, HostedClip,
                PatchClipsClipIdRequest, UploadClipResponse,
            },
            tags::ClipsTag,
        },
        error::WayclipError,
        nutype::ClipNameSanitised,
        query::{FullQueryWeb, PaginatedResponseWeb},
    },
    settings::output::VideoFormat,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use itertools::Itertools;
use reqwest::Method;
use std::{mem::discriminant, path::PathBuf};
use tokio::{fs, io::AsyncReadExt};

const UPLOAD_CHUNK_SIZE: usize = 5 * 1000 * 1000; // 5 MB 

/// This struct is a submodule of WayclipClient, responsbile for HTTP calls related to Clips
/// managements.
#[derive(Clone)]
pub struct ClipsHttpClient {
    client: WayclipClient,
}

/// This enum allows us to pass in only the required fields to be editted, when calling `patch`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatchClipField {
    /// Edit clip visibility
    Visibility(ClipVisibility),
    /// Edit name
    Name(String),
    /// Edit tags
    Tags(Vec<ClipsTag>),
    /// Edit game type
    Game(Option<Game>),
}

impl ClipsHttpClient {
    /// Create a new client
    pub fn new(api_url: url::Url) -> Result<Self, WayclipError> {
        let client = WayclipClient::new(api_url)?;
        Ok(Self { client })
    }

    /// Call to query the currently logged-in user's clips
    pub async fn query_me_clips(
        &mut self,
        query: FullQueryWeb,
    ) -> Result<PaginatedResponseWeb<HostedClip>, WayclipError> {
        let response: WayclipResponse<PaginatedResponseWeb<ClipsResponse>> = self
            .client
            .with_credentials()
            .await?
            .with_body(&query)?
            .send_call(Method::POST, "/users/me/clips/search")
            .await?;

        response.into_inner()
    }

    /// Call to delete a specific clip that user owns
    pub async fn delete(&mut self, clip_id: &str) -> Result<(), WayclipError> {
        self.client
            .with_credentials()
            .await?
            .send_call::<()>(Method::DELETE, &format!("/clips/{}", clip_id))
            .await?;
        Ok(())
    }

    /// Call to patch a clip, but only w specific clips
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
            .with_body(&body)?
            .send_call(Method::PATCH, &format!("/clips/{}", clip_id))
            .await?;

        Ok(())
    }

    /// Updated method for TUS uploads. This allows to resume them and more benefits.
    /// Works in 3 stages:
    /// 1. Get upload token from POST api.wayclip.com/clips -> This gives us <upload_url>
    /// 2. Create TUSD resource via POST <upload_url>
    /// 3. Stream data via PATCH <upload_url>
    pub async fn upload(
        &mut self,
        video_format: VideoFormat,
        video_path: &PathBuf,
        metadata: ClipsNewMetadata,
    ) -> Result<HostedClip, WayclipError> {
        let file_metadata = fs::metadata(video_path).await?;

        let response: WayclipResponse<UploadClipResponse> = self
            .client
            .with_credentials()
            .await?
            .with_body(&metadata)?
            .send_call(Method::POST, "/clips")
            .await?;

        let upload_info = response.into_inner()?;

        let file_size = file_metadata.len();
        let file_name = metadata.name.into_inner();
        let mime_string = video_format.get_mime_str().to_string();

        let create_headers = self.generate_create_tus_headers(
            &upload_info.upload_token,
            &upload_info.metadata.clip_id,
            &file_name,
            &mime_string,
            file_size,
        );

        let create: WayclipResponse<()> = self
            .client
            .with_credentials()
            .await?
            .with_headers(create_headers)
            .send_url(Method::POST, upload_info.upload_url.clone())
            .await?;

        let location = create
            .headers
            .get("Location")
            .ok_or_else(|| {
                WayclipError::Validation("Missing Location header in tus response".into())
            })?
            .to_str()
            .map_err(|e| WayclipError::Validation(e.to_string().into()))?;

        let patch_url = if let Ok(parsed) = url::Url::parse(location) {
            parsed
        } else {
            let mut base = upload_info.upload_url.clone();
            if !base.path().ends_with('/') {
                base.set_path(&format!("{}/", base.path()));
            }
            base.join(location)
                .map_err(|e| WayclipError::Validation(e.to_string().into()))?
        };

        // use chunks and offsets to stream certain parts of the file
        let mut file = fs::File::open(video_path).await?;
        let mut offset: u64 = 0;
        while offset < file_size {
            let to_read = std::cmp::min(UPLOAD_CHUNK_SIZE as u64, file_size - offset) as usize;
            let mut chunk = vec![0u8; to_read];
            file.read_exact(&mut chunk).await?;

            let patch_headers =
                self.generate_patch_tus_headers(offset, chunk.len(), &upload_info.upload_token);

            let patch: WayclipResponse<()> = self
                .client
                .with_credentials()
                .await?
                .with_headers(patch_headers)
                .with_raw_body(chunk)
                .send_url(Method::PATCH, patch_url.clone())
                .await?;

            if let Some(new_offset_header) = patch.headers.get("Upload-Offset")
                && let Ok(new_offset_str) = new_offset_header.to_str()
                && let Ok(new_offset) = new_offset_str.parse::<u64>()
            {
                offset = new_offset;
                continue;
            }

            offset += to_read as u64;
        }

        Ok(upload_info.metadata)
    }

    fn generate_patch_tus_headers(
        &self,
        offset: u64,
        chunk_len: usize,
        upload_token: &str,
    ) -> Vec<(String, String)> {
        vec![
            ("Tus-Resumable".into(), "1.0.0".into()),
            ("Upload-Offset".into(), offset.to_string()),
            (
                "Content-Type".into(),
                "application/offset+octet-stream".into(),
            ),
            ("Content-Length".into(), chunk_len.to_string()),
            ("Authorization".into(), format!("Bearer {}", upload_token)),
            ("Upload-Token".into(), upload_token.to_string()),
        ]
    }

    fn generate_create_tus_headers(
        &self,
        upload_token: &str,
        clip_id: &str,
        file_name: &str,
        file_type: &str,
        file_size: u64,
    ) -> Vec<(String, String)> {
        let tus_metadata = [
            format!("upload_token {}", STANDARD.encode(upload_token.as_bytes())),
            format!("clip_id {}", STANDARD.encode(clip_id.as_bytes())),
            format!("filename {}", STANDARD.encode(file_name.as_bytes())),
            format!("filetype {}", STANDARD.encode(file_type.as_bytes())),
        ]
        .join(",");

        vec![
            ("Tus-Resumable".into(), "1.0.0".into()),
            ("Upload-Length".into(), file_size.to_string()),
            ("Upload-Metadata".into(), tus_metadata),
            ("Authorization".into(), format!("Bearer {}", upload_token)),
            ("Upload-Token".into(), upload_token.to_string()),
        ]
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
