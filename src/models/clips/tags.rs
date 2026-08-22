use crate::models::{error::WayclipError, nutype::TagNameSanitised};
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ClipsClipTagRequest {
    pub name: TagNameSanitised,
    pub color: ClipsClipTagColor,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "openapi",
    derive(utoipa::ToSchema),
    derive(sea_orm::FromJsonQueryResult)
)]
pub struct ClipsClipTagResponse {
    pub name: String,
    pub color: ClipsClipTagColor,
}

// Makes it easier to know which one (response/request) to use inside App
pub type ClipsTag = ClipsClipTagResponse;

impl From<ClipsClipTagRequest> for ClipsClipTagResponse {
    fn from(value: ClipsClipTagRequest) -> Self {
        Self {
            name: value.name.into_inner(),
            color: value.color,
        }
    }
}

impl TryFrom<ClipsClipTagResponse> for ClipsClipTagRequest {
    type Error = WayclipError;
    fn try_from(value: ClipsClipTagResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            name: TagNameSanitised::try_new(value.name)?,
            color: value.color,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumIter)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum ClipsClipTagColor {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Grey,
}

impl FromStr for ClipsClipTagColor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "reset" => Self::Reset,
            "red" => Self::Red,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "blue" => Self::Blue,
            "magenta" => Self::Magenta,
            "cyan" => Self::Cyan,
            "grey" => Self::Grey,
            _ => return Err("Invalid color".to_string()),
        })
    }
}

impl ClipsClipTagColor {
    pub fn get_colored_string(&self) -> ColoredString {
        match self {
            Self::Red => "Red".red(),
            Self::Green => "Green".green(),
            Self::Yellow => "Yellow".yellow(),
            Self::Blue => "Blue".blue(),
            Self::Magenta => "Magenta".magenta(),
            Self::Cyan => "Cyan".cyan(),
            Self::Grey => "Grey".normal(),
            Self::Reset => "Normal".normal(),
        }
    }
}

impl Default for ClipsClipTagResponse {
    fn default() -> Self {
        ClipsClipTagResponse {
            name: String::from(""),
            color: ClipsClipTagColor::Reset,
        }
    }
}

impl ClipsClipTagResponse {
    pub fn get_colored_string(&self) -> ColoredString {
        self.color.get_colored_string()
    }
}
