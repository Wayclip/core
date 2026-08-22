use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDiscovery {
    pub enabled: bool,
    pub poll_interval_s: u64,
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
