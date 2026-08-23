use crate::{
    models::error::WayclipError,
    settings::{
        api::ApiSettings, discovery::GameDiscovery, migration::SettingsMigrate,
        notifications::NotificationSettings, output::OutputSettings, recording::RecordingSettings,
        registry::SettingsRegistry, shortcut::ShortcutsSettings, tray::TraySettings,
    },
};
use dirs::config_dir;
use semver::Version;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    fmt::Display,
    fs::{read_to_string, write},
    path::PathBuf,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_CONFIG_PATH: &str = "wayclip/config.json";

/// API settings
pub mod api;
/// Game discovery settings
pub mod discovery;
/// Module responsible for migration between versions
pub mod migration;
/// Notification settings
pub mod notifications;
/// Daemon output settings
pub mod output;
/// Daemon recording settings
pub mod recording;
/// Module responsible for fetching inside and mutating the settings
pub mod registry;
/// Module responsbile for extracting and setting up the schema from `settings.toml`
pub mod schema;
/// Shortcut settings
pub mod shortcut;
/// Tray settings
pub mod tray;

/// The main settings struct that is stored inside users `~/.config` directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    /// The current version of the settings, will be used to migrate to newer ones (e.g. to add
    /// fields)
    pub v: semver::Version,
    /// API settings
    pub api: ApiSettings,
    /// Daemon recording settings
    pub recording: RecordingSettings,
    /// Daemon output settings
    pub output: OutputSettings,
    /// Shortcut settings
    pub shortcuts: ShortcutsSettings,
    /// Game discovery settings
    pub game_discovery: GameDiscovery,
    /// Notification settings
    pub notification: NotificationSettings,
    /// Tray settings
    pub tray: TraySettings,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            // let it panic, since VERSION is directly from Cargo.toml
            v: Version::parse(VERSION).expect("Could not parse VERSION"),
            api: ApiSettings::default(),
            recording: RecordingSettings::default(),
            output: OutputSettings::default(),
            shortcuts: ShortcutsSettings::default(),
            game_discovery: GameDiscovery::default(),
            notification: NotificationSettings::default(),
            tray: TraySettings::default(),
        }
    }
}

/// Struct to first extracted version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ver {
    v: semver::Version,
}

impl UserSettings {
    /// Basically this checks if a config file already exists
    /// If it does read from it, otherwise create default.
    pub fn load() -> Result<Self, WayclipError> {
        let file = Self::config_path()?;

        if file.exists() {
            let output = read_to_string(file)?;
            let string = output.as_str();

            let v = serde_json::from_str::<Ver>(string)
                .map_err(|e| WayclipError::Config(format!("Config file corrupted: {e}").into()))?
                .v;

            let mut config: serde_json::Value = serde_json::from_str(&output)?;
            SettingsMigrate::migrate(&mut config, v, Version::parse(VERSION)?)?;
            let settings: UserSettings = serde_json::from_value(config)?;
            Ok(settings)
        } else {
            let settings = Self::default();
            Self::save_to_local_disk(&settings)?;
            Ok(settings)
        }
    }

    /// Stores the current UserSettings onto the disk
    pub fn save_to_local_disk(&self) -> Result<(), WayclipError> {
        let file = Self::config_path()?;
        // Ensure all parent folders exist
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let pretty_json = serde_json::to_string_pretty(self)?;
        write(file, pretty_json)?;
        Ok(())
    }

    /// Updates a set key with a serde_json::Value
    pub fn set_value(&mut self, key: &str, value: serde_json::Value) -> Result<(), WayclipError> {
        let (_, def) = SettingsRegistry::find_by_key(key)
            .ok_or_else(|| WayclipError::NotFound(format!("No such key: {key}").into()))?;

        let parsed_value = SettingsRegistry::parse_raw_value(key, value)?;

        let target_path = if def.location.is_empty() {
            def.field_name.clone()
        } else {
            format!("{}.{}", def.location, def.field_name)
        };

        *self = SettingsRegistry::set_value(self, &target_path, parsed_value)?;
        Ok(())
    }

    /// Updates a set key with a &str
    pub fn set_str(&mut self, key: &str, value: &str) -> Result<(), WayclipError> {
        let (_, def) = SettingsRegistry::find_by_key(key)
            .ok_or_else(|| WayclipError::NotFound(format!("No such key: {key}").into()))?;

        let parsed_value = SettingsRegistry::parse_raw_str(key, value)?;

        let target_path = if def.location.is_empty() {
            def.field_name.clone()
        } else {
            format!("{}.{}", def.location, def.field_name)
        };

        *self = SettingsRegistry::set_value(self, &target_path, parsed_value)?;
        Ok(())
    }

    /// Gets a certain key's value
    pub fn get<R: DeserializeOwned + Send + 'static + Display>(
        &self,
        key: &str,
    ) -> Result<R, WayclipError> {
        let (_, def) = SettingsRegistry::find_by_key(key)
            .ok_or_else(|| WayclipError::NotFound(format!("No such key: {key}").into()))?;

        let target_path = if def.location.is_empty() {
            def.field_name.clone()
        } else {
            format!("{}.{}", def.location, def.field_name)
        };

        SettingsRegistry::get_value(self, &target_path)
    }

    // pub fn with_recording_settings(&mut self, recording_settings: RecordingSettings) {
    //     self.recording = recording_settings;
    // }
    // pub fn with_output_settings(&mut self, output_settings: OutputSettings) {
    //     self.output = output_settings;
    // }
    // pub fn with_api_settings(&mut self, api_settings: ApiSettings) {
    //     self.api = api_settings;
    // }
    // pub fn with_shortcut_settings(&mut self, shortcut_settings: ShortcutsSettings) {
    //     self.shortcuts = shortcut_settings;
    // }
    // pub fn with_game_discovery_settings(&mut self, discovery_settings: GameDiscovery) {
    //     self.game_discovery = discovery_settings;
    // }

    /// Hardcoded path at which the `config.json` file is located
    pub fn config_path() -> Result<PathBuf, WayclipError> {
        Ok(config_dir()
            .expect("No config dir found..")
            .join(DEFAULT_CONFIG_PATH))
    }
}
