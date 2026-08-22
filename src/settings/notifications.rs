use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundsNotification {
    pub on_save_success: bool,
    pub on_save_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageNotification {
    pub on_daemon_start: bool,
    pub on_daemon_stop: bool,
    pub on_save_success: bool,
    pub on_save_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationSettings {
    pub message: MessageNotification,
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
