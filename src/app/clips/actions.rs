use crate::{models::clips::unified::SelectedClip, settings::UserSettings};
use strum_macros::Display;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Display)]
pub enum MixedClipActions {
    View,
    #[strum(to_string = "Copy Name")]
    CopyName,
    #[strum(to_string = "Copy Path")]
    CopyPath,
    #[strum(to_string = "Open Path")]
    OpenPath,
    Upload,
    Like,
    Trim,
    #[strum(to_string = "Change Name")]
    ChangeName,
    #[strum(to_string = "Change Tag")]
    ChangeTag,
    #[strum(to_string = "Change Game")]
    ChangeGame,
    #[strum(to_string = "Change Visibility")]
    ChangeVisibility,
    Delete,
}

impl MixedClipActions {
    pub fn get_both() -> Vec<Self> {
        vec![
            MixedClipActions::Delete,
            MixedClipActions::ChangeTag,
            MixedClipActions::ChangeName,
            MixedClipActions::ChangeGame,
            MixedClipActions::CopyName,
        ]
    }

    pub fn get_local() -> Vec<Self> {
        vec![
            MixedClipActions::Like,
            MixedClipActions::View,
            MixedClipActions::Delete,
            MixedClipActions::ChangeTag,
            MixedClipActions::ChangeName,
            MixedClipActions::Trim,
            MixedClipActions::ChangeGame,
            MixedClipActions::CopyPath,
            MixedClipActions::CopyName,
            MixedClipActions::OpenPath,
        ]
    }

    pub fn get_hosted() -> Vec<Self> {
        vec![
            MixedClipActions::View,
            MixedClipActions::Delete,
            MixedClipActions::ChangeTag,
            MixedClipActions::ChangeName,
            MixedClipActions::ChangeGame,
            MixedClipActions::ChangeVisibility,
            MixedClipActions::CopyName,
        ]
    }

    pub fn get_available_actions(selected_clip: &SelectedClip) -> Vec<Self> {
        let settings = UserSettings::load().unwrap_or_default();

        let mut items = match selected_clip {
            SelectedClip::Both(_, _) => Self::get_both(),
            SelectedClip::Local(clip) => {
                let mut actions = Self::get_local();

                // If clip is not uploaded (linked) yet and api is enabled, add option to upload
                if clip.uploaded_at.is_none() && clip.uploaded_id.is_none() && settings.api.enabled
                {
                    actions.push(MixedClipActions::Upload)
                }

                actions
            }
            SelectedClip::Hosted(_) => Self::get_hosted(),
        };

        items.sort();
        items
    }
}
