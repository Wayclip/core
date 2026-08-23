use crate::models::error::WayclipError;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, str::FromStr};

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WayclipControllerButton {
    South,
    East,
    North,
    West,
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
    Select,
    Start,
    Mode,
    LeftThumb,
    RightThumb,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

impl FromStr for WayclipControllerButton {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "south" | "a" | "cross" => Ok(Self::South),
            "east" | "b" | "circle" => Ok(Self::East),
            "north" | "y" | "triangle" => Ok(Self::North),
            "west" | "x" | "square" => Ok(Self::West),
            "lefttrigger2" | "lt" | "l2" => Ok(Self::LeftTrigger2),
            "lefttrigger" | "lb" | "l1" => Ok(Self::LeftTrigger),
            "righttrigger2" | "rt" | "r2" => Ok(Self::RightTrigger2),
            "righttrigger" | "rb" | "r1" => Ok(Self::RightTrigger),
            "select" | "back" => Ok(Self::Select),
            "start" => Ok(Self::Start),
            "mode" | "guide" | "home" => Ok(Self::Mode),
            "leftthumb" | "l3" | "left_thumb" => Ok(Self::LeftThumb),
            "rightthumb" | "r3" | "right_thumb" => Ok(Self::RightThumb),
            "dpadup" | "dpad_up" => Ok(Self::DPadUp),
            "dpaddown" | "dpad_down" => Ok(Self::DPadDown),
            "dpadleft" | "dpad_left" => Ok(Self::DPadLeft),
            "dpadright" | "dpad_right" => Ok(Self::DPadRight),
            other => Err(WayclipError::Validation(
                format!("Invalid controller button: {other}").into(),
            )),
        }
    }
}

impl From<WayclipControllerButton> for gilrs::Button {
    fn from(value: WayclipControllerButton) -> Self {
        match value {
            WayclipControllerButton::South => Self::South,
            WayclipControllerButton::East => Self::East,
            WayclipControllerButton::North => Self::North,
            WayclipControllerButton::West => Self::West,
            WayclipControllerButton::LeftTrigger => Self::LeftTrigger,
            WayclipControllerButton::LeftTrigger2 => Self::LeftTrigger2,
            WayclipControllerButton::RightTrigger => Self::RightTrigger,
            WayclipControllerButton::RightTrigger2 => Self::RightTrigger2,
            WayclipControllerButton::Select => Self::Select,
            WayclipControllerButton::Start => Self::Start,
            WayclipControllerButton::Mode => Self::Mode,
            WayclipControllerButton::LeftThumb => Self::LeftThumb,
            WayclipControllerButton::RightThumb => Self::RightThumb,
            WayclipControllerButton::DPadUp => Self::DPadUp,
            WayclipControllerButton::DPadDown => Self::DPadDown,
            WayclipControllerButton::DPadLeft => Self::DPadLeft,
            WayclipControllerButton::DPadRight => Self::DPadRight,
        }
    }
}

/// The combination of buttons that are held together to trigger an action.
/// On controller there are less restrictions about what keys and their number have to be pressed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WayclipControllerCombo {
    /// The buttons themselves
    pub buttons: Vec<WayclipControllerButton>,
}

impl WayclipControllerCombo {
    /// Method to know if the combo entered will trigger the combination
    pub fn is_satisfied(&self, held: &HashSet<gilrs::Button>) -> bool {
        !self.buttons.is_empty() && self.buttons.iter().all(|b| held.contains(&(*b).into()))
    }
}

impl std::fmt::Display for WayclipControllerCombo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = self.buttons.iter().map(|b| format!("{b:?}")).collect();
        write!(f, "{}", parts.join("+"))
    }
}

impl FromStr for WayclipControllerCombo {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buttons = s
            .split('+')
            .map(|p| p.trim().parse())
            .collect::<Result<Vec<_>, Self::Err>>()?;
        Ok(Self { buttons })
    }
}

impl TryFrom<String> for WayclipControllerCombo {
    type Error = WayclipError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl From<WayclipControllerCombo> for String {
    fn from(val: WayclipControllerCombo) -> Self {
        val.to_string()
    }
}
