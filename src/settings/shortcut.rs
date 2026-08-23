use crate::models::input::{
    controller::WayclipControllerCombo,
    keyboard::{WayclipKeyCode, WayclipKeyCombo, WayclipKeyModifiers},
};
use serde::{Deserialize, Serialize};

const DEFAULT_SHORTCUT_KEY_CODE: WayclipKeyCode = WayclipKeyCode::Char('c');
const DEFAULT_SHORTCUT_KEY_MODIFIER: WayclipKeyModifiers = WayclipKeyModifiers::ALT;

/// Shortcut settings daemon will follow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutsSettings {
    /// WayclipKeyCombo since supports a combination of alt, win, ctrl + any key
    pub save_clip: WayclipKeyCombo,
    /// WayclipControllerCombo is an addition combo, which works for any combination of a STANDARD controller layout
    pub save_clip_controller: Option<WayclipControllerCombo>,
}

impl Default for ShortcutsSettings {
    fn default() -> Self {
        Self {
            save_clip: WayclipKeyCombo::new(
                DEFAULT_SHORTCUT_KEY_CODE,
                DEFAULT_SHORTCUT_KEY_MODIFIER,
            ),
            save_clip_controller: None,
        }
    }
}
