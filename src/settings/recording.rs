use crate::models::error::WayclipError;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

pub const DEFAULT_LENGTH_SECONDS: u64 = 120;
pub const DEFAULT_RESOLUTION: (u64, u64) = (1920, 1080);
pub const DEFAULT_VIDEO_CODEC: VideoCodec = VideoCodec::H264(CodecType::NVIDIA);
pub const DEFAULT_FPS: u64 = 30;
pub const DEFAULT_BITRATE_KBPS: u64 = 15000;
pub const DEFAULT_AUDIO_CODEC: AudioCodec = AudioCodec::Opus;
pub const DEFAULT_MICROPHONE_LEVEL: f64 = 0.75;
pub const DEFAULT_BACKGROUND_LEVEL: f64 = 0.50;
pub const DEFAULT_MICROPHONE_ENABLED: bool = true;
pub const DEFAULT_BACKGROUND_ENABLED: bool = true;
pub const DEFAULT_AUDIO_SAMPLE_RATE: u64 = 48000;
pub const MIN_RESOLUTION_WIDTH: u64 = 1;
pub const MAX_RESOLUTION_WIDTH: u64 = 7680;
pub const MIN_RESOLUTION_HEIGHT: u64 = 1;
pub const MAX_RESOLUTION_HEIGHT: u64 = 4320;
pub const MIN_BITRATE_KBPS: u64 = 300;
pub const MAX_BITRATE_KBPS: u64 = 10000000;
pub const MIN_AUDIO_LEVEL: f64 = 0.0;
pub const MAX_AUDIO_LEVEL: f64 = 1.0;
pub const MIN_FPS: u64 = 1;
pub const MAX_FPS: u64 = 1000;
pub const ALLOWED_AUDIO_SAMPLE_RATES_HZ: &[u64] = &[8000, 16000, 22050, 32000, 44100, 48000, 96000];

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecordingSettings {
    pub video: VideoSettings,
    pub audio: AudioSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    pub length_seconds: u64,
    pub resolution: Resolution,
    pub fps: Fps,
    pub codec: VideoCodec,
    pub bitrate_kbps: Bitrate,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            length_seconds: DEFAULT_LENGTH_SECONDS,
            resolution: Resolution::default(),
            fps: Fps::default(),
            codec: DEFAULT_VIDEO_CODEC,
            bitrate_kbps: Bitrate::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Fps(pub u64);

impl Default for Fps {
    fn default() -> Self {
        Self(DEFAULT_FPS)
    }
}

impl FromStr for Fps {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fps: u64 = s.parse()?;
        if !(MIN_FPS..=MAX_FPS).contains(&fps) {
            return Err(WayclipError::Validation(
                format!(
                    "Fps must be within the range {} to {} FPS",
                    MIN_FPS, MAX_FPS
                )
                .into(),
            ));
        }

        Ok(Self(fps))
    }
}

impl Display for Fps {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}fps", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resolution {
    pub width: u64,
    pub height: u64,
}

impl Resolution {
    pub fn from_tuple<U>(tuple: (U, U)) -> Self
    where
        U: Into<u64>,
    {
        Self {
            width: tuple.0.into(),
            height: tuple.1.into(),
        }
    }

    pub fn to_tuple(&self) -> (u64, u64) {
        (self.width, self.height)
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            width: DEFAULT_RESOLUTION.0,
            height: DEFAULT_RESOLUTION.1,
        }
    }
}

impl FromStr for Resolution {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 2 {
            return Err(WayclipError::Validation(
                "Resolution must be in 'WIDTHxHEIGHT' format (e.g. 1920x1080)".into(),
            ));
        }

        let width = parts[0]
            .parse()
            .map_err(|_| WayclipError::Validation("Invalid resolution width".into()))?;
        let height = parts[1]
            .parse()
            .map_err(|_| WayclipError::Validation("Invalid resolution height".into()))?;

        if width < MIN_RESOLUTION_WIDTH
            || height < MIN_RESOLUTION_HEIGHT
            || width > MAX_RESOLUTION_WIDTH
            || height > MAX_RESOLUTION_HEIGHT
        {
            return Err(WayclipError::Validation(
                format!(
                    "Resolution must be within the range {}x{} and {}x{} pixels",
                    MIN_RESOLUTION_WIDTH,
                    MIN_RESOLUTION_HEIGHT,
                    MAX_RESOLUTION_WIDTH,
                    MAX_RESOLUTION_HEIGHT
                )
                .into(),
            ));
        }

        Ok(Self { width, height })
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bitrate(pub u64);

impl Default for Bitrate {
    fn default() -> Self {
        Self(DEFAULT_BITRATE_KBPS)
    }
}

impl FromStr for Bitrate {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kbps: u64 = s.parse()?;

        if !(MIN_BITRATE_KBPS..=MAX_BITRATE_KBPS).contains(&kbps) {
            return Err(WayclipError::Validation(
                format!(
                    "Bitrate value must be within the range {} to {} kbps",
                    MIN_BITRATE_KBPS, MAX_BITRATE_KBPS
                )
                .into(),
            ));
        }

        Ok(Self(kbps))
    }
}

impl Display for Bitrate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}kbps", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum CodecType {
    // Requires proprietary drivers and some gstreamer package
    // https://gstreamer.freedesktop.org/documentation/nvcodec/index.html
    NVIDIA,
    // Requires libva and supported driver
    VAAPI,
    // One of the gstreamer-packages has it
    Software,
}

impl std::fmt::Display for CodecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodecType::NVIDIA => write!(f, "nvidia"),
            CodecType::Software => write!(f, "software"),
            CodecType::VAAPI => write!(f, "vaapi"),
        }
    }
}

impl FromStr for CodecType {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, WayclipError> {
        match s.to_lowercase().as_str() {
            "nvidia" | "nv" => Ok(CodecType::NVIDIA),
            "vaapi" => Ok(CodecType::VAAPI),
            "software" | "sw" => Ok(CodecType::Software),
            _ => Err(WayclipError::Validation("Invalid codec type".into())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VideoCodec {
    H264(CodecType),
    H265(CodecType),
    AV1(CodecType),
}

impl FromStr for VideoCodec {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, WayclipError> {
        let (codec_name, codec_ty) = s
            .split_once(':')
            .ok_or_else(|| WayclipError::Validation("Expected format is <codec>:<type>".into()))?;
        let codec_type = codec_ty.parse()?;
        match codec_name.to_lowercase().as_str() {
            "h264" => Ok(VideoCodec::H264(codec_type)),
            "h265" => Ok(VideoCodec::H265(codec_type)),
            "av1" => Ok(VideoCodec::AV1(codec_type)),
            _ => Err(WayclipError::Validation("Invalid codec name".into())),
        }
    }
}

impl std::fmt::Display for VideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoCodec::AV1(codec_type) => {
                write!(f, "av1:{codec_type}")
            }
            VideoCodec::H265(codec_type) => {
                write!(f, "h265:{codec_type}")
            }
            VideoCodec::H264(codec_type) => {
                write!(f, "h264:{codec_type}")
            }
        }
    }
}

impl VideoCodec {
    pub fn get_parser(&self) -> &str {
        match self {
            VideoCodec::H264(_) => "h264parse",
            VideoCodec::H265(_) => "h265parse",
            VideoCodec::AV1(_) => "av1parse",
        }
    }

    pub fn get_backend(&self) -> &CodecType {
        match self {
            VideoCodec::H264(t) | VideoCodec::H265(t) | VideoCodec::AV1(t) => t,
        }
    }

    pub fn get_encoder(&self) -> &str {
        match self {
            // https://gstreamer.freedesktop.org/documentation/nvcodec/nvh264enc.html?gi-language=rust
            VideoCodec::H264(CodecType::NVIDIA) => "nvh264enc",
            // https://gstreamer.freedesktop.org/documentation/nvcodec/nvh265enc.html?gi-language=rust
            VideoCodec::H265(CodecType::NVIDIA) => "nvh265enc",
            // https://gstreamer.freedesktop.org/documentation/nvcodec/nvav1enc.html?gi-language=rust
            VideoCodec::AV1(CodecType::NVIDIA) => "nvav1enc",

            // https://gstreamer.freedesktop.org/documentation/va/vah264enc.html?gi-language=rust
            VideoCodec::H264(CodecType::VAAPI) => "vah264enc",
            // https://gstreamer.freedesktop.org/documentation/va/vah265enc.html?gi-language=rust
            VideoCodec::H265(CodecType::VAAPI) => "vah265enc",
            // https://gstreamer.freedesktop.org/documentation/va/vaav1enc.html?gi-language=rust
            VideoCodec::AV1(CodecType::VAAPI) => "vaav1enc",

            // https://gstreamer.freedesktop.org/documentation/x264/index.html?gi-language=rust
            VideoCodec::H264(CodecType::Software) => "x264enc",
            // https://gstreamer.freedesktop.org/documentation/x265/index.html?gi-language=rust
            VideoCodec::H265(CodecType::Software) => "x265enc",
            // https://gstreamer.freedesktop.org/documentation/aom/av1enc.html?gi-language=rust
            VideoCodec::AV1(CodecType::Software) => "av1enc",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub sample_rate_hz: SampleRate,
    pub codec: AudioCodec,
    pub microphone: AudioNode,
    pub background: AudioNode,
}

impl Default for AudioSettings {
    fn default() -> Self {
        // On startup -> empty strings
        // When pipewire comes to life -> replace
        Self {
            sample_rate_hz: SampleRate::default(),
            codec: DEFAULT_AUDIO_CODEC,
            microphone: AudioNode::new(
                String::new(),
                DEFAULT_MICROPHONE_LEVEL,
                DEFAULT_MICROPHONE_ENABLED,
            ),
            background: AudioNode::new(
                String::new(),
                DEFAULT_BACKGROUND_LEVEL,
                DEFAULT_BACKGROUND_ENABLED,
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleRate(pub u64);

impl Default for SampleRate {
    fn default() -> Self {
        Self(DEFAULT_AUDIO_SAMPLE_RATE)
    }
}

impl FromStr for SampleRate {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hz: u64 = s.parse()?;

        if !ALLOWED_AUDIO_SAMPLE_RATES_HZ.contains(&hz) {
            return Err(WayclipError::Validation(
                format!(
                    "The audio sample rate may only be one of the following: {:?}",
                    ALLOWED_AUDIO_SAMPLE_RATES_HZ
                )
                .into(),
            ));
        }

        Ok(Self(hz))
    }
}

impl Display for SampleRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}hz", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioNode {
    pub level: f64,
    pub node_name: String,
    pub enabled: bool,
}

impl AudioNode {
    pub fn new(node_name: String, level: f64, enabled: bool) -> Self {
        Self {
            level,
            node_name,
            enabled,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioLevel(pub f64);

impl FromStr for AudioLevel {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let level: f64 = s.parse()?;

        if !(MIN_AUDIO_LEVEL..=MAX_AUDIO_LEVEL).contains(&level) {
            return Err(WayclipError::Validation(
                format!(
                    "The audio level must be within the range {} to {}",
                    MIN_AUDIO_LEVEL, MAX_AUDIO_LEVEL
                )
                .into(),
            ));
        }

        Ok(Self(level))
    }
}

impl Display for AudioLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AudioCodec {
    Opus,
    AAC,
    MP3,
}

impl FromStr for AudioCodec {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, WayclipError> {
        match s.to_lowercase().as_str() {
            "opus" => Ok(AudioCodec::Opus),
            "aac" => Ok(AudioCodec::AAC),
            "mp3" => Ok(AudioCodec::MP3),
            _ => Err(WayclipError::Validation("Invalid codec type".into())),
        }
    }
}

impl std::fmt::Display for AudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AudioCodec::Opus => "opus",
            AudioCodec::MP3 => "mp3",
            AudioCodec::AAC => "aac",
        };

        write!(f, "{s}")
    }
}

impl AudioCodec {
    pub fn get_encoder(&self) -> &str {
        match self {
            AudioCodec::Opus => "opusenc",
            AudioCodec::AAC => "avenc_aac",
            AudioCodec::MP3 => "lamemp3enc",
        }
    }

    pub fn get_parser(&self) -> &str {
        match self {
            AudioCodec::Opus => "opusparse",
            AudioCodec::AAC => "aacparse",
            AudioCodec::MP3 => "mpegaudioparse",
        }
    }
}
