use crate::models::error::WayclipError;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf, str::FromStr, time::Duration};

pub const DEFAULT_CONFIG_PATH: &str = "wayclip/config.json";
pub const DEFAULT_PREVIEW_DIRECTORY: &str = "Videos/wayclip/";
pub const DEFAULT_CLIP_DIRECTORY: &str = "Videos/wayclip/";
pub const DEFAULT_METADATA_DIRECTORY: &str = "Videos/wayclip/";
pub const DEFAULT_OUTPUT_NAME_FORMAT: &str = "wayclip_{game}_%Y-%m-%d_%H-%M-%S";
pub const DEFAULT_OUTPUT_VIDEO_FORMAT: VideoFormat = VideoFormat::MKV;
// 0 means unlimited
pub const DEFAULT_MAX_SIZE_MB: u64 = 0;
pub const DEFAULT_MAX_CLIPS: u64 = 0;
pub const DEFAULT_PRUNE: Prune = Prune::Disabled;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSettings {
    pub name_format: String,
    pub clip_directory: Directory,
    pub preview_directory: Directory,
    pub metadata_directory: Directory,
    pub limit: LimitSettings,
    pub video_format: VideoFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directory(pub PathBuf);

impl FromStr for Directory {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = if s.starts_with("~/") {
            if let Some(home) = dirs::home_dir() {
                home.join(s)
            } else {
                PathBuf::from(s)
            }
        } else {
            PathBuf::from(s)
        };
        if !dir.exists() || !dir.is_dir() {
            return Err(WayclipError::NotFound(
                "Provided path doesnt exist or is not a directory".into(),
            ));
        }
        Ok(Self(dir))
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitSettings {
    pub max_size_mb: u64,
    pub max_clips: u64,
    pub prune: Prune,
}

impl Default for LimitSettings {
    fn default() -> Self {
        Self {
            max_size_mb: DEFAULT_MAX_SIZE_MB,
            max_clips: DEFAULT_MAX_CLIPS,
            prune: DEFAULT_PRUNE,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Prune {
    Disabled,
    SizeMb(u64),
    Clips(usize),
    // To be clear, the S means seconds and isnt a typo...
    DurationS(u64),
    Age(Age),
}

impl Display for Prune {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled => write!(f, "Disabled"),
            Self::SizeMb(mb) => write!(f, "After {}MB in dir", mb),
            Self::Clips(num) => write!(f, "After {} clips in dir", num),
            Self::DurationS(s) => write!(f, "Longer than {}s", s),
            Self::Age(age) => write!(f, "Older than {} from now", age),
        }
    }
}

impl FromStr for Prune {
    type Err = WayclipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Disabled" {
            return Ok(Prune::Disabled);
        }

        if let Some(rest) = s.strip_prefix("After ") {
            if let Some(num_str) = rest.strip_suffix("MB in dir") {
                let num = num_str.parse::<u64>()?;
                return Ok(Prune::SizeMb(num));
            }
            if let Some(num_str) = rest.strip_suffix(" clips in dir") {
                let num = num_str.parse::<usize>()?;
                return Ok(Prune::Clips(num));
            }
        }

        // Handle DurationS
        if let Some(rest) = s.strip_prefix("Longer than ") {
            if let Some(num_str) = rest.strip_suffix("s") {
                let num = num_str.parse::<u64>()?;
                return Ok(Prune::DurationS(num));
            }
        }

        // Handle Age
        if let Some(rest) = s.strip_prefix("Older than ") {
            if let Some(age_str) = rest.strip_suffix(" from now") {
                let age = age_str.parse::<Age>()?;
                return Ok(Prune::Age(age));
            }
        }

        Err(WayclipError::Validation(
            format!("Unrecognized Prune format: '{}'", s).into(),
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Age {
    pub months: u64,
    pub weeks: u64,
    pub days: u64,
    pub hours: u64,
}

impl Age {
    pub fn get_duration(&self) -> Duration {
        let (months, weeks, days, hours) = (self.months, self.weeks, self.days, self.hours);
        Duration::from_hours(hours + days * 24 + weeks * 7 * 24 + months * 30 * 24)
    }
}

impl Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Months, {} Weeks, {} Days, {} Hours",
            self.months, self.weeks, self.days, self.hours
        )
    }
}

impl FromStr for Age {
    type Err = WayclipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 4 {
            return Err(WayclipError::Validation(
                format!("Invalid Age format, expected 4 parts: {}", s).into(),
            ));
        }

        let parse_part = |part: &str, expected_label: &str| -> Result<u64, Self::Err> {
            let part = part.trim();
            let (num_str, label) = part.split_once(' ').ok_or_else(|| {
                WayclipError::NotFound(format!("Missing space in part '{}'", part).into())
            })?;

            if !label.eq_ignore_ascii_case(expected_label) {
                return Err(WayclipError::Validation(
                    format!("Expected '{}', found '{}'", expected_label, label).into(),
                ));
            }

            let res = num_str.parse::<u64>()?;
            Ok(res)
        };

        Ok(Age {
            months: parse_part(parts[0], "Months")?,
            weeks: parse_part(parts[1], "Weeks")?,
            days: parse_part(parts[2], "Days")?,
            hours: parse_part(parts[3], "Hours")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VideoFormat {
    MP4,
    MKV,
    MPEGTS,
}

impl FromStr for VideoFormat {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mp4" => Ok(VideoFormat::MP4),
            "mkv" => Ok(VideoFormat::MKV),
            "mpegts" => Ok(VideoFormat::MPEGTS),
            _ => Err(WayclipError::Validation("Invalid format type".into())),
        }
    }
}

impl std::fmt::Display for VideoFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            VideoFormat::MPEGTS => "mpegts",
            VideoFormat::MKV => "mkv",
            VideoFormat::MP4 => "mp4",
        };

        write!(f, "{s}")
    }
}

impl VideoFormat {
    pub fn get_mux(&self) -> &str {
        match self {
            VideoFormat::MKV => "matroskamux",
            VideoFormat::MP4 => "mp4mux",
            VideoFormat::MPEGTS => "mpegtsmux",
        }
    }

    pub fn get_extension(&self) -> &str {
        match self {
            VideoFormat::MKV => "mkv",
            VideoFormat::MP4 => "mp4",
            VideoFormat::MPEGTS => "ts",
        }
    }

    pub fn get_mime_str(&self) -> &str {
        match self {
            VideoFormat::MP4 => "video/mp4",
            VideoFormat::MKV => "video/x-matroska",
            VideoFormat::MPEGTS => "video/mp2t",
        }
    }
}

impl Default for OutputSettings {
    fn default() -> Self {
        // Idk how else to handle that they dont have a home dir since like there isnt any fallback
        let clip_directory = dirs::home_dir()
            .expect("No home directory was found..")
            .join(DEFAULT_CLIP_DIRECTORY);
        let preview_directory = dirs::home_dir()
            .expect("No home directory was found..")
            .join(DEFAULT_PREVIEW_DIRECTORY);
        let metadata_directory = dirs::home_dir()
            .expect("No home directory was found..")
            .join(DEFAULT_METADATA_DIRECTORY);
        Self {
            name_format: String::from(DEFAULT_OUTPUT_NAME_FORMAT),
            video_format: DEFAULT_OUTPUT_VIDEO_FORMAT,
            clip_directory: Directory(clip_directory),
            preview_directory: Directory(preview_directory),
            metadata_directory: Directory(metadata_directory),
            limit: LimitSettings::default(),
        }
    }
}
