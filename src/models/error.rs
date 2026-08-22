use rodio::{DeviceSinkError, decoder::DecoderError};
use std::{
    borrow::Cow,
    char::ParseCharError,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WayclipError {
    // some daemon errors
    #[error("Ring error: {0}")]
    Ring(Cow<'static, str>),

    #[error("Remux error: {0}")]
    Remux(Cow<'static, str>),

    #[error("Tray error: {0}")]
    Tray(Cow<'static, str>),

    #[error("Pipewire error: {0}")]
    Pipewire(Cow<'static, str>),

    #[error("Discovery error: {0}")]
    Discovery(Cow<'static, str>),

    #[error("Audio error: {0}")]
    Audio(Cow<'static, str>),

    #[error("Screencast error: {0}")]
    Screencast(Cow<'static, str>),

    #[error("Video error: {0}")]
    Video(Cow<'static, str>),

    #[error("Video error: {0}")]
    Watcher(Cow<'static, str>),

    #[error("Fatal error: {0}")]
    Fatal(Cow<'static, str>),

    // cli errors
    #[error("CLI error: {0}")]
    CLI(Cow<'static, str>),

    // general errors
    #[error("Config error: {0}")]
    Config(Cow<'static, str>),

    #[error("API error: {0}")]
    Api(Cow<'static, str>),

    // library-specific errors
    #[error("Reqwest error occurred: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(Cow<'static, str>),

    #[error("Not Found error: {0}")]
    NotFound(Cow<'static, str>),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("FFmpeg error occurred: {0}")]
    FFmpeg(#[from] rust_ffmpeg::Error),

    #[error("URL Parse error occurred: {0}")]
    URL(#[from] url::ParseError),

    #[error("Keyring error occurred: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("Task join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Parse Char error: {0}")]
    ParseChar(#[from] ParseCharError),

    #[error("Parse Int error: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("Parse Bool error: {0}")]
    ParseBool(#[from] ParseBoolError),

    #[error("Parse Float error: {0}")]
    ParseFloat(#[from] ParseFloatError),

    #[error("Semver Parse error: {0}")]
    Semver(#[from] semver::Error),

    #[error("GLib error: {0}")]
    GLib(#[from] gstreamer::glib::Error),

    #[error("GLibBool error: {0}")]
    GLibBool(#[from] gstreamer::glib::BoolError),

    #[error("PadLink error: {0}")]
    PadLink(#[from] gstreamer::PadLinkError),

    #[error("StateChange error: {0}")]
    StateChange(#[from] gstreamer::StateChangeError),

    #[error("DeviceSink error: {0}")]
    DeviceSink(#[from] DeviceSinkError),

    #[error("DecoderError error: {0}")]
    DecoderError(#[from] DecoderError),

    #[error("ZBus error: {0}")]
    ZBus(#[from] zbus::Error),

    #[error("Fdo error: {0}")]
    Fdo(#[from] zbus::fdo::Error),

    #[error("Var error: {0}")]
    Var(#[from] std::env::VarError),

    #[error("GlobakHotKey error: {0}")]
    GlobalHK(#[from] wayclip_global_hotkey::Error),

    #[cfg(feature = "errors")]
    #[error("Gilrs error: {0}")]
    Gilrs(Box<gilrs::Error>),

    #[cfg(feature = "errors")]
    #[error("Portal error: {0}")]
    Portal(Box<ashpd::Error>),
}

#[cfg(feature = "errors")]
impl From<gilrs::Error> for WayclipError {
    fn from(value: gilrs::Error) -> Self {
        Self::Gilrs(Box::new(value))
    }
}

#[cfg(feature = "errors")]
impl From<ashpd::Error> for WayclipError {
    fn from(value: ashpd::Error) -> Self {
        Self::Portal(Box::new(value))
    }
}
