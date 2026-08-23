use crate::models::clips::{hosted::HostedClip, local::LocalClip};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// Unified clip, allowing us to collect both local and hosted clips to showcase user
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnifiedClip {
    /// General name of clip (usually gotten from local)
    pub name: String,
    /// When clip was created (file metadata)
    pub created_at: DateTime<Utc>,
    /// LocalClip model, if it exists
    pub local: Option<LocalClip>,
    /// HostedClip model, if it exists
    pub hosted: Option<HostedClip>,
}

/// Type of a unified clip
#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum UnifiedClipType {
    /// Only local is present
    #[strum(to_string = "Local")]
    LocalOnly,
    /// Only hosted is present
    #[strum(to_string = "Hosted")]
    HostedOnly,
    /// Both hosted and local are present
    #[strum(to_string = "Local & Hosted")]
    Both,
    /// Neither are present (unreachable)
    #[strum(to_string = "Not Available")]
    None,
}

/// The clip to manage after user found correct one
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectedClip {
    /// Manage local
    Local(LocalClip),
    /// Manage hosted
    Hosted(HostedClip),
    /// Manage both
    Both(LocalClip, HostedClip),
}

/// The type of the selected clip
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectedClipType {
    /// Local
    Local,
    /// Hosted
    Hosted,
    /// Both
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
