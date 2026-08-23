use serde::{Deserialize, Serialize};

/// Game Discovery settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDiscovery {
    /// If daemon should discover game
    pub enabled: bool,
    /// The interval with which daemon will discover
    pub poll_interval_s: u64,
    /// If daemon should report to discord rich presence
    pub discord_rich_presence: bool,
}

impl Default for GameDiscovery {
    fn default() -> Self {
        Self {
            enabled: true,
            poll_interval_s: 20,
            discord_rich_presence: true,
        }
    }
}
