use serde::{Deserialize, Serialize};

pub const DEFAULT_TRAY_ENABLED: bool = true;
pub const DEFAULT_TRAY_SHOW_LOGO: bool = true;
pub const DEFAULT_TRAY_SHOW_SAVE_CLIP: bool = true;
pub const DEFAULT_TRAY_SHOW_RESTART: bool = true;
pub const DEFAULT_TRAY_SHOW_EXIT: bool = true;
pub const DEFAULT_TRAY_SHOW_STATUS: bool = true;
pub const DEFAULT_TRAY_SHOW_STATS: bool = true;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraySettings {
    pub enabled: bool,
    pub show_logo: bool,
    pub show_save_clip: bool,
    pub show_restart: bool,
    pub show_exit: bool,
    pub show_status: bool,
    pub show_stats: bool,
}

impl Default for TraySettings {
    fn default() -> Self {
        Self {
            enabled: DEFAULT_TRAY_ENABLED,
            show_logo: DEFAULT_TRAY_SHOW_LOGO,
            show_save_clip: DEFAULT_TRAY_SHOW_SAVE_CLIP,
            show_restart: DEFAULT_TRAY_SHOW_RESTART,
            show_exit: DEFAULT_TRAY_SHOW_EXIT,
            show_status: DEFAULT_TRAY_SHOW_STATUS,
            show_stats: DEFAULT_TRAY_SHOW_STATS,
        }
    }
}
