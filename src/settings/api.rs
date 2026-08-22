use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;

pub const DEFAULT_API_URL: &str = "https://api.wayclip.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSettings {
    pub enabled: bool,
    pub url: Url,
}

impl Default for ApiSettings {
    fn default() -> Self {
        let url = Url::from_str(DEFAULT_API_URL).expect("Could not parse URL");
        Self { enabled: true, url }
    }
}
