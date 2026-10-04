use serde::{Deserialize, Serialize};

/// Settings for sending sounds on events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundsNotification {
    /// Should sound play on clip save success
    pub on_save_success: bool,
    /// Should sound play on clip save error
    pub on_save_error: bool,
}

/// Settings for sending notifications on events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageNotification {
    /// Send notification when daemon starts
    pub on_daemon_start: bool,
    /// Send notification when daemon stops
    pub on_daemon_stop: bool,
    /// Send notification when clip save success
    pub on_save_success: bool,
    /// Send notification when clip save error
    pub on_save_error: bool,
}

/// The notification settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationSettings {
    /// When to send notification message
    pub message: MessageNotification,
    /// When to play sounds
    pub sounds: SoundsNotification,
}

impl Default for MessageNotification {
    fn default() -> Self {
        Self {
            on_save_error: true,
            on_save_success: true,
            on_daemon_stop: false,
            on_daemon_start: false,
        }
    }
}

impl Default for SoundsNotification {
    fn default() -> Self {
        Self {
            on_save_success: true,
            on_save_error: true,
        }
    }
}
