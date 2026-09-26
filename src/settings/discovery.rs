use serde::{Deserialize, Serialize};

/// Game Discovery settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDiscovery {
    /// If daemon should discover game
    pub enabled: bool,
    /// The interval with which daemon will discover
    pub poll_interval_s: u64,
    /// settings responsible for discord rich presence reporting and configuration
    pub discord_rich_presence: DiscordRichPresence,
}

/// settings responsible for discord rich presence reporting and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordRichPresence {
    /// If daemon should report to discord rich presence
    pub enabled: bool,
    /// Display 'Get Wayclip' button?
    pub enable_promo: bool,
    /// If user is logged, should we display the profile URL?
    pub display_profile: bool,
    /// User can overwrite all of default states
    pub custom: Option<CustomDiscordRichPresence>,
}

/// User will have a choice to set a custom discord rich presence status
/// It will still show up as wayclip, but user will be able to add custom state & details
/// We can also support custom formatting like %game%, %name%, etc...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomDiscordRichPresence {
    /// The main string, state, that will be displayed
    pub state: String,
    /// We will let details to support the following custom strings that will get auto-formatted
    /// %time% -> Current UTC Time
    /// %started_at% -> UTC Time of start of current sesssion
    /// %playing_for% -> Display how long playing for in HH:MM
    /// %last_clip% -> Display since when last clip in HH:MM
    /// %game% -> Current Users Game
    /// %buffer_length% -> How long their buffer is
    /// %fps% -> FPS Setting
    /// %res% -> Res Setting
    /// %session_clips% -> Clips made this session
    /// %username% -> Current username of user (only if logged in)
    pub details: String,
    /// The custom activity type
    pub activity_type: CustomDiscordRichPresenceActivityType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum CustomDiscordRichPresenceActivityType {
    Playing,
    Watching,
    Listening,
    Competing,
}

impl Default for GameDiscovery {
    fn default() -> Self {
        Self {
            enabled: true,
            poll_interval_s: 20,
            discord_rich_presence: DiscordRichPresence {
                enabled: true,
                enable_promo: true,
                display_profile: true,
                custom: None,
            },
        }
    }
}
