use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::LazyLock};

const SCHEMA_TOML: &str = include_str!("../../assets/settings.toml");
/// The global schema of the settings.toml
pub static SCHEMA: LazyLock<GlobalSchema> = LazyLock::new(|| {
    toml::from_str(SCHEMA_TOML).expect("Failed to parse assets/settings.toml. Check TOML syntax.")
});

/// Global schema struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalSchema {
    /// All the settings key-value pairs
    pub settings: HashMap<String, SettingsDefinition>,
    /// Migration version changes
    pub versions: Vec<VersionEntry>,
}

/// Schema for defining a setting
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsDefinition {
    /// The key
    pub key: String,
    /// Location (parent)
    pub location: String,
    /// Field name
    pub field_name: String,
    /// Type of data stored
    pub r#type: String,
    /// The default value
    pub default: serde_json::Value,
    /// Version the field introduced in
    pub introduced_in: String,
}

/// A migration version entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionEntry {
    /// The version to be used in
    pub version: semver::Version,
    /// The changes introduced in the version
    pub changes: Vec<MigrationChange>,
}

/// A type of migration change
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum MigrationChange {
    /// Either inserting a new field
    Insert {
        /// With a setting_id, linking to already defined settings in the TOML
        setting_id: String,
    },
    /// Moving a field from one place to another without changing the value
    Move {
        /// From
        from_path: String,
        /// To
        to_path: String,
        /// What field
        field: String,
    },
    /// Removing a field
    Remove {
        /// Parent (location)
        location: String,
        /// Field name itself
        field_name: String,
    },
}
