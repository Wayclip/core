use crate::{models::clips::unified::SelectedClip, settings::UserSettings};
use strum_macros::Display;

/// `MixedClipActions` is an enum which contains all the actions a user can take on a clip. No
/// matter if the clip is classified as local, hosted, or other.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Display)]
pub enum MixedClipActions {
    /// `View` action corresponds to viewing the clip either via the browser or via `mpv` or similar
    /// tool
    View,
    /// `CopyName` action uses the os clipboard to copy the name of the clip. The name to be copied
    /// is the one stored inside the metadata file, not the file name.
    #[strum(to_string = "Copy Name")]
    CopyName,
    /// `CopyPath` action uses the os clipboard to copy the full path of the clip.
    #[strum(to_string = "Copy Path")]
    CopyPath,
    /// `OpenPath` action uses the native default application to open the the path of where clip is
    /// stored
    #[strum(to_string = "Open Path")]
    OpenPath,
    /// `Upload` action uploads the current clip to the Wayclip API. This will only work for Local
    /// clips and if the API is enabled in settings.
    Upload,
    /// `Like` action allows the user to mark a clip as Liked. This will show up the CLI/GUI is a
    /// heart, but currently has no Querying capability.
    Like,
    /// `Trim` action allows the user to trim (change the length) of their clip by specifying the
    /// start and end positions, as well as method of trimming (either Replace/Copy)
    Trim,
    /// `ChangeName` action allows user to change name of their clip
    #[strum(to_string = "Change Name")]
    ChangeName,
    /// `ChangeName` action allows user to change the tags assosciated with their clip. What
    /// specific fields inside the tag to change are handled by a different process
    #[strum(to_string = "Change Tag")]
    /// `ChangeTag` action allows user to change the tags assosciated with their clip. What
    /// specific fields inside the tag to change are handled by a different process
    ChangeTag,
    /// `ChangeGame` action allows user to change the game assosciated with their clip
    #[strum(to_string = "Change Game")]
    ChangeGame,
    /// `ChangeGame` action allows user to change the clip visibility. This works only for Hosted
    /// clips, allowing to control who can see the clip
    #[strum(to_string = "Change Visibility")]
    ChangeVisibility,
    /// `Delete` action allows user to remove clip either locally or from the API
    Delete,
}

impl MixedClipActions {
    /// This method gives actions that both the Hosted and Local clips can use
    pub fn get_both() -> Vec<Self> {
        vec![
            MixedClipActions::Delete,
            MixedClipActions::ChangeTag,
            MixedClipActions::ChangeName,
            MixedClipActions::ChangeGame,
            MixedClipActions::CopyName,
        ]
    }

    /// This method gives actions that only the Local clips can use. This method does not return
    /// `Upload`, since that depends on the users' settings
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

    /// This method gives actions that Hosted clips can use
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

    /// This method returns the actions that the SelectedClip can use. Here is where we add the
    /// `Upload` option too.
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
