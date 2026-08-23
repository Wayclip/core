use serde::{Deserialize, Serialize};

const DEFAULT_TRAY_ENABLED: bool = true;
const DEFAULT_TRAY_SHOW_LOGO: bool = true;
const DEFAULT_TRAY_SHOW_SAVE_CLIP: bool = true;
const DEFAULT_TRAY_SHOW_RESTART: bool = true;
const DEFAULT_TRAY_SHOW_EXIT: bool = true;
const DEFAULT_TRAY_SHOW_STATUS: bool = true;
const DEFAULT_TRAY_SHOW_STATS: bool = true;

/// The Tray settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraySettings {
    /// Enable the tray itself
    pub enabled: bool,
    /// Show logo at the top
    pub show_logo: bool,
    /// Show save clip button
    pub show_save_clip: bool,
    /// Show restart daemon button
    pub show_restart: bool,
    /// Show exit daemon button
    pub show_exit: bool,
    /// Show daemon status
    pub show_status: bool,
    /// Show daemon stats
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
