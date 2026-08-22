use crate::{
    models::{error::WayclipError, input::keyboard::WayclipKeyCombo},
    settings::{
        output::{Directory, Prune, VideoFormat},
        recording::{AudioCodec, AudioLevel, Bitrate, Fps, Resolution, SampleRate, VideoCodec},
        schema::{SCHEMA, SettingsDefinition},
    },
};
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Display;
use url::Url;

/// Registery responsible for interactions with the TOML
pub struct SettingsRegistry;

impl SettingsRegistry {
    pub fn find_by_key(key: &str) -> Option<(&'static String, &'static SettingsDefinition)> {
        SCHEMA
            .settings
            .iter()
            .find(|(_, def)| def.key.eq_ignore_ascii_case(key))
    }

    pub fn parse_raw_value(
        key: &str,
        value: serde_json::Value,
    ) -> Result<serde_json::Value, WayclipError> {
        let (_, def) =
            Self::find_by_key(key).ok_or_else(|| WayclipError::NotFound("No such key".into()))?;

        match def.r#type.as_str() {
            "bool" => {
                let b: bool = serde_json::from_value(value)?;
                Ok(serde_json::to_value(b)?)
            }
            "u64" => {
                let u: u64 = serde_json::from_value(value)?;
                Ok(serde_json::to_value(u)?)
            }
            "f64" => {
                let f: f64 = serde_json::from_value(value)?;
                Ok(serde_json::to_value(f)?)
            }
            // TODO: Perhaps path checks if exists?
            "string" | "path" => Ok(value),
            "directory" => {
                let dir: Directory = serde_json::from_value(value)?;
                Ok(serde_json::to_value(dir)?)
            }
            "bitrate" => {
                let bitrate: Bitrate = serde_json::from_value(value)?;
                Ok(serde_json::to_value(bitrate)?)
            }
            "url" => {
                let url: Url = serde_json::from_value(value)?;
                Ok(serde_json::to_value(url)?)
            }
            "fps" => {
                let fps: Fps = serde_json::from_value(value)?;
                Ok(serde_json::to_value(fps)?)
            }
            "sample_rate" => {
                let sample_rate: SampleRate = serde_json::from_value(value)?;
                Ok(serde_json::to_value(sample_rate)?)
            }
            "audio_level" => {
                let audio_level: AudioLevel = serde_json::from_value(value)?;
                Ok(serde_json::to_value(audio_level)?)
            }
            "resolution" => {
                let res: Resolution = serde_json::from_value(value)?;
                Ok(serde_json::to_value(res)?)
            }
            "video_codec" => {
                let video_codec: VideoCodec = serde_json::from_value(value)?;
                Ok(serde_json::to_value(video_codec)?)
            }
            "audio_codec" => {
                let audio_codec: AudioCodec = serde_json::from_value(value)?;
                Ok(serde_json::to_value(audio_codec)?)
            }
            "video_format" => {
                let video_format: VideoFormat = serde_json::from_value(value)?;
                Ok(serde_json::to_value(video_format)?)
            }
            "prune" => {
                let prune: Prune = serde_json::from_value(value)?;
                Ok(serde_json::to_value(prune)?)
            }
            "key_combo" => {
                let key_combo: WayclipKeyCombo = serde_json::from_value(value)?;
                Ok(serde_json::to_value(key_combo)?)
            }
            _ => Err(WayclipError::NotFound(
                "No type handler found for this key".into(),
            )),
        }
    }

    pub fn parse_raw_str(key: &str, value: &str) -> Result<serde_json::Value, WayclipError> {
        let (_, def) =
            Self::find_by_key(key).ok_or_else(|| WayclipError::NotFound("No such key".into()))?;

        match def.r#type.as_str() {
            "bool" => {
                let b: bool = value.parse()?;
                Ok(serde_json::to_value(b)?)
            }
            "u64" => {
                let u: u64 = value.parse()?;
                Ok(serde_json::to_value(u)?)
            }
            "f64" => {
                let f: f64 = value.parse()?;
                Ok(serde_json::to_value(f)?)
            }
            // TODO: Perhaps path checks if exists?
            "string" | "path" => Ok(serde_json::to_value(value)?),
            "directory" => {
                let dir: Directory = value.parse()?;
                Ok(serde_json::to_value(dir)?)
            }
            "bitrate" => {
                let bitrate: Bitrate = value.parse()?;
                Ok(serde_json::to_value(bitrate)?)
            }
            "url" => {
                let url: Url = value.parse()?;
                Ok(serde_json::to_value(url)?)
            }
            "fps" => {
                let fps: Fps = value.parse()?;
                Ok(serde_json::to_value(fps)?)
            }
            "sample_rate" => {
                let sample_rate: SampleRate = value.parse()?;
                Ok(serde_json::to_value(sample_rate)?)
            }
            "audio_level" => {
                let audio_level: AudioLevel = value.parse()?;
                Ok(serde_json::to_value(audio_level)?)
            }
            "resolution" => {
                let res: Resolution = value.parse()?;
                Ok(serde_json::to_value(res)?)
            }
            "video_codec" => {
                let video_codec: VideoCodec = value.parse()?;
                Ok(serde_json::to_value(video_codec)?)
            }
            "audio_codec" => {
                let audio_codec: AudioCodec = value.parse()?;
                Ok(serde_json::to_value(audio_codec)?)
            }
            "video_format" => {
                let video_format: VideoFormat = value.parse()?;
                Ok(serde_json::to_value(video_format)?)
            }
            "prune" => {
                let prune: Prune = value.parse()?;
                Ok(serde_json::to_value(prune)?)
            }
            "key_combo" => {
                let key_combo: WayclipKeyCombo = value.parse()?;
                Ok(serde_json::to_value(key_combo)?)
            }
            _ => Err(WayclipError::NotFound(
                "No type handler found for this key".into(),
            )),
        }
    }

    pub fn get_value<T: Serialize, R: DeserializeOwned + Send + 'static + Display>(
        settings: &T,
        path: &str,
    ) -> Result<R, WayclipError> {
        let json_value = serde_json::to_value(settings)?;

        let mut current = &json_value;
        for part in path.split('.') {
            current = current
                .get(part)
                .ok_or_else(|| WayclipError::NotFound(path.to_string().into()))?;
        }

        Ok(serde_json::from_value(current.to_owned())?)
    }

    pub fn set_value<T: Serialize + DeserializeOwned>(
        settings: &T,
        path: &str,
        new_val: serde_json::Value,
    ) -> Result<T, WayclipError> {
        let mut json_value = serde_json::to_value(settings)?;

        let parts: Vec<&str> = path.split('.').collect();
        let (last_key, parent_keys) = parts
            .split_last()
            .ok_or_else(|| WayclipError::NotFound(path.to_string().into()))?;

        let mut current = &mut json_value;
        for part in parent_keys {
            current = current
                .get_mut(part)
                .ok_or_else(|| WayclipError::NotFound(path.to_string().into()))?;
        }

        let target_node = current
            .get_mut(last_key)
            .ok_or_else(|| WayclipError::NotFound(path.to_string().into()))?;

        *target_node = new_val;

        let updated_settings: T = serde_json::from_value(json_value)
            .map_err(|err| WayclipError::Validation(err.to_string().into()))?;

        Ok(updated_settings)
    }
}
