use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;

const DEFAULT_API_URL: &str = "https://api.wayclip.com";

/// API settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSettings {
    /// If the APP is allowed to make network requests
    pub enabled: bool,
    /// The URL at which the API is located
    pub url: Url,
}

impl Default for ApiSettings {
    fn default() -> Self {
        let url = Url::from_str(DEFAULT_API_URL).expect("Could not parse URL");
        Self { enabled: true, url }
    }
}
