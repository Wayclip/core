use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::LazyLock};

pub const SCHEMA_TOML: &str = include_str!("../../static/settings.toml");
pub static SCHEMA: LazyLock<GlobalSchema> = LazyLock::new(|| {
    toml::from_str(SCHEMA_TOML).expect("Failed to parse static/settings.toml. Check TOML syntax.")
});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalSchema {
    pub settings: HashMap<String, SettingsDefinition>,
    pub versions: Vec<VersionEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsDefinition {
    pub key: String,
    pub location: String,
    pub field_name: String,
    pub r#type: String,
    pub default: serde_json::Value,
    pub introduced_in: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionEntry {
    pub version: semver::Version,
    pub changes: Vec<MigrationChange>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum MigrationChange {
    Insert {
        setting_id: String,
    },
    Move {
        from_path: String,
        to_path: String,
        field: String,
    },
    Remove {
        location: String,
        field_name: String,
    },
}
