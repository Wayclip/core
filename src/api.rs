use crate::models::{HostedClipInfo, UserProfile};
use crate::Settings;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{multipart, Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};
use uuid::Uuid;

#[derive(Debug)]
pub enum ApiClientError {
    Unauthorized,
    NotFound,
    ApiError { status: u16, message: String },
    RequestError(reqwest::Error),
    SerializationError(serde_json::Error),
    Io(std::io::Error),
    Config(anyhow::Error),
}
impl fmt::Display for ApiClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiClientError::Unauthorized => write!(f, "Unauthorized: Please log in."),
            ApiClientError::NotFound => write!(f, "The requested resource was not found."),
            ApiClientError::ApiError { status, message } => {
                write!(f, "API Error ({status}): {message}")
            }
            ApiClientError::RequestError(e) => write!(f, "Request Error: {e}"),
            ApiClientError::SerializationError(e) => write!(f, "Serialization Error: {e}"),
            ApiClientError::Io(e) => write!(f, "File I/O Error: {e}"),
            ApiClientError::Config(e) => write!(f, "Configuration Error: {e}"),
        }
    }
}
impl Error for ApiClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ApiClientError::RequestError(e) => Some(e),
            ApiClientError::SerializationError(e) => Some(e),
            ApiClientError::Io(e) => Some(e),
            ApiClientError::Config(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
impl From<anyhow::Error> for ApiClientError {
    fn from(err: anyhow::Error) -> Self {
        ApiClientError::Config(err)
    }
}
impl From<reqwest::Error> for ApiClientError {
    fn from(err: reqwest::Error) -> Self {
        ApiClientError::RequestError(err)
    }
}
impl From<serde_json::Error> for ApiClientError {
    fn from(err: serde_json::Error) -> Self {
        ApiClientError::SerializationError(err)
    }
}
impl From<std::io::Error> for ApiClientError {
    fn from(err: std::io::Error) -> Self {
        ApiClientError::Io(err)
    }
}

async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, ApiClientError> {
    match response.status() {
        StatusCode::OK | StatusCode::CREATED => Ok(response.json::<T>().await?),
        StatusCode::UNAUTHORIZED => Err(ApiClientError::Unauthorized),
        StatusCode::NOT_FOUND => Err(ApiClientError::NotFound),
        status => {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not retrieve error message.".to_string());
            Err(ApiClientError::ApiError {
                status: status.as_u16(),
                message,
            })
        }
    }
}

pub async fn get_api_client() -> Result<Client, ApiClientError> {
    let settings = Settings::load().await?;
    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(token) = settings.auth_token {
        headers.insert("Authorization", format!("Bearer {token}").parse().unwrap());
    }
    Ok(Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(300))
        .build()?)
}

pub async fn login(token: String) -> Result<()> {
    let mut settings = Settings::load().await?;
    settings.auth_token = Some(token);
    settings.save().await?;
    Ok(())
}

pub async fn logout() -> Result<()> {
    let mut settings = Settings::load().await?;
    settings.auth_token = None;
    settings.save().await?;
    Ok(())
}

pub async fn get_current_user() -> Result<UserProfile, ApiClientError> {
    let client = get_api_client().await?;
    let settings = Settings::load().await?;
    let response = client
        .get(format!("{}/api/me", settings.api_url))
        .send()
        .await?;
    handle_response(response).await
}

pub async fn get_hosted_clips_index() -> Result<Vec<HostedClipInfo>, ApiClientError> {
    let client = get_api_client().await?;
    let settings = Settings::load().await?;
    let response = client
        .get(format!("{}/api/clips/index", settings.api_url))
        .send()
        .await?;
    handle_response(response).await
}

pub async fn share_clip(client: &Client, clip_path: &Path) -> Result<String, ApiClientError> {
    let settings = Settings::load().await?;
    let file = File::open(clip_path).await?;
    let file_size = file.metadata().await?.len();
    let file_name = clip_path.file_name().unwrap().to_str().unwrap().to_string();

    let preflight_response: serde_json::Value = handle_response(
        client
            .post(format!("{}/api/share/begin", settings.api_url))
            .json(&serde_json::json!({
                "file_name": file_name.clone(),
                "file_size": file_size,
            }))
            .send()
            .await?,
    )
    .await?;

    let upload_id =
        preflight_response["upload_id"]
            .as_str()
            .ok_or_else(|| ApiClientError::ApiError {
                status: 500,
                message: "Server did not return an upload_id".to_string(),
            })?;
    let final_url = preflight_response["url"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, ETA {eta})")
        .unwrap()
        .progress_chars("=>-"));

    let pb_clone = pb.clone();
    let progress_stream = FramedRead::new(file, BytesCodec::new()).map(move |res| match res {
        Ok(bytes) => {
            pb_clone.inc(bytes.len() as u64);
            Ok(bytes)
        }
        Err(e) => Err(e),
    });

    let body = reqwest::Body::wrap_stream(progress_stream);
    let part = multipart::Part::stream(body)
        .file_name(file_name)
        .mime_str("video/mp4")?;
    let form = multipart::Form::new().part("video", part);

    let upload_url = format!("{}/api/upload/{}", settings.api_url, upload_id);
    let response = client.post(&upload_url).multipart(form).send().await?;

    if !response.status().is_success() {
        pb.finish_with_message("Upload failed!");
        return Err(ApiClientError::ApiError {
            status: response.status().as_u16(),
            message: response.text().await.unwrap_or_default(),
        });
    }

    pb.finish_with_message("✔ Upload complete. Processing...");
    Ok(final_url)
}

pub async fn delete_clip(client: &Client, clip_id: Uuid) -> Result<(), ApiClientError> {
    let settings = Settings::load().await?;
    let response = client
        .delete(format!("{}/api/clip/{}", settings.api_url, clip_id))
        .send()
        .await?;

    match response.status() {
        StatusCode::NO_CONTENT => Ok(()),
        StatusCode::UNAUTHORIZED => Err(ApiClientError::Unauthorized),
        StatusCode::NOT_FOUND => Err(ApiClientError::NotFound),
        status => {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not retrieve error message.".to_string());
            Err(ApiClientError::ApiError {
                status: status.as_u16(),
                message,
            })
        }
    }
}
