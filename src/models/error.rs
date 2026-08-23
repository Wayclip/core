use rodio::{DeviceSinkError, decoder::DecoderError};
use std::{
    borrow::Cow,
    char::ParseCharError,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};
use thiserror::Error;

/// An enum which contains all the possible errors
/// Most of the types here are made to be converted from external third-party error types
#[derive(Error, Debug)]
pub enum WayclipError {
    // Daemon types
    /// An ambiguous 'ring' error
    #[error("Ring error: {0}")]
    Ring(Cow<'static, str>),

    /// An ambiguous 'remux' error
    #[error("Remux error: {0}")]
    Remux(Cow<'static, str>),

    /// An ambiguous 'tray' error
    #[error("Tray error: {0}")]
    Tray(Cow<'static, str>),

    /// An ambiguous 'pipewire' error
    #[error("Pipewire error: {0}")]
    Pipewire(Cow<'static, str>),

    /// An ambiguous 'discovery' error
    #[error("Discovery error: {0}")]
    Discovery(Cow<'static, str>),

    /// An ambiguous 'audio' error
    #[error("Audio error: {0}")]
    Audio(Cow<'static, str>),

    /// An ambiguous 'screencast' error
    #[error("Screencast error: {0}")]
    Screencast(Cow<'static, str>),

    /// An ambiguous 'video' error
    #[error("Video error: {0}")]
    Video(Cow<'static, str>),

    /// An ambiguous 'watcher' error
    #[error("Video error: {0}")]
    Watcher(Cow<'static, str>),

    /// An ambiguous 'fatal' error
    #[error("Fatal error: {0}")]
    Fatal(Cow<'static, str>),

    // CLI errors
    /// A general 'CLI' error
    #[error("CLI error: {0}")]
    CLI(Cow<'static, str>),

    /// A general 'config' error
    #[error("Config error: {0}")]
    Config(Cow<'static, str>),

    /// A general 'API' error
    #[error("API error: {0}")]
    Api(Cow<'static, str>),

    /// A general 'Validation' error
    #[error("Validation error: {0}")]
    Validation(Cow<'static, str>),

    /// A general 'NotFound' error
    #[error("Not Found error: {0}")]
    NotFound(Cow<'static, str>),

    // Library-specific errors
    /// Handles errors thrown by reqwest
    #[error("Reqwest error occurred: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Handles errors thrown by std::io
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Handles errors thrown by serde_json
    #[error("JSON error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Handles errors thrown by regex
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    /// Handles errors thrown by rust_ffmpeg
    #[error("FFmpeg error occurred: {0}")]
    FFmpeg(#[from] rust_ffmpeg::Error),

    /// Handles errors thrown by url
    #[error("URL Parse error occurred: {0}")]
    URL(#[from] url::ParseError),

    /// Handles errors thrown by keyring
    #[error("Keyring error occurred: {0}")]
    Keyring(#[from] keyring::Error),

    /// Handles errors thrown by tokio::task::JoinError
    #[error("Task join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    /// Handles errors thrown by core::char::conver::ParseCharError
    #[error("Parse Char error: {0}")]
    ParseChar(#[from] ParseCharError),

    /// Handles errors thrown by core::num::error::ParseIntError
    #[error("Parse Int error: {0}")]
    ParseInt(#[from] ParseIntError),

    /// Handles errors thrown by core::str:error::ParseBoolError
    #[error("Parse Bool error: {0}")]
    ParseBool(#[from] ParseBoolError),

    /// Handles errors thrown by core::num::float_parse::ParseFloatError
    #[error("Parse Float error: {0}")]
    ParseFloat(#[from] ParseFloatError),

    /// Handles errors thrown by semver
    #[error("Semver Parse error: {0}")]
    Semver(#[from] semver::Error),

    /// Handles errors thrown by gstreamer::glib::Error
    #[error("GLib error: {0}")]
    GLib(#[from] gstreamer::glib::Error),

    /// Handles errors thrown by gstreamer::glib::BoolError
    #[error("GLibBool error: {0}")]
    GLibBool(#[from] gstreamer::glib::BoolError),

    /// Handles errors thrown by gstreamer::PadLinkError
    #[error("PadLink error: {0}")]
    PadLink(#[from] gstreamer::PadLinkError),

    /// Handles errors thrown by gstreamer::StateChangeError
    #[error("StateChange error: {0}")]
    StateChange(#[from] gstreamer::StateChangeError),

    /// Handles errors thrown by rodio::stream
    #[error("DeviceSink error: {0}")]
    DeviceSink(#[from] DeviceSinkError),

    /// Handles errors thrown by rodio::decoder
    #[error("DecoderError error: {0}")]
    DecoderError(#[from] DecoderError),

    /// Handles errors thrown by zbus
    #[error("ZBus error: {0}")]
    ZBus(#[from] zbus::Error),

    /// Handles errors thrown by zbus::fdo
    #[error("Fdo error: {0}")]
    Fdo(#[from] zbus::fdo::Error),

    /// Handles errors thrown by std::env
    #[error("Var error: {0}")]
    Var(#[from] std::env::VarError),

    /// Handles errors thrown by wayclip_global_hotkey
    #[error("GlobakHotKey error: {0}")]
    GlobalHK(#[from] wayclip_global_hotkey::Error),

    /// Handles errors thrown by Gilrs
    /// Locked behind `errors` feature
    #[cfg(feature = "errors")]
    #[error("Gilrs error: {0}")]
    Gilrs(Box<gilrs::Error>),

    /// Handles errors thrown by ashpd
    /// Locked behind `errors` feature
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
