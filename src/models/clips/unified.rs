use crate::models::clips::{hosted::HostedClip, local::LocalClip};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnifiedClip {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub local: Option<LocalClip>,
    pub hosted: Option<HostedClip>,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum UnifiedClipType {
    #[strum(to_string = "Local")]
    LocalOnly,
    #[strum(to_string = "Hosted")]
    HostedOnly,
    #[strum(to_string = "Local & Hosted")]
    Both,
    #[strum(to_string = "Not Available")]
    None,
}

// tbh im not sure why this error occurs or why its important
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectedClip {
    Local(LocalClip),
    Hosted(HostedClip),
    Both(LocalClip, HostedClip),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectedClipType {
    Local,
    Hosted,
    Both,
}

impl From<UnifiedClipType> for SelectedClipType {
    fn from(value: UnifiedClipType) -> Self {
        match value {
            UnifiedClipType::HostedOnly => Self::Hosted,
            UnifiedClipType::LocalOnly => Self::Local,
            // its never gonna get to this if you use the enum correctly.
            _ => unreachable!(),
        }
    }
}
