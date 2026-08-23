use crate::{
    client::clips::{ClipsHttpClient, PatchClipField},
    models::{clips::hosted::HostedClip, error::WayclipError},
    settings::UserSettings,
};

impl HostedClip {
    /// `patch` accepts a vector of changes instead of passing in all the values. This way we can
    /// change one value at the time without clutter.
    /// This method is basically a wrapper around ClipsHttpClient & HostedClip where we handle
    /// settings, creation of the client and sending the call
    pub async fn patch(&self, new_values: Vec<PatchClipField>) -> Result<(), WayclipError> {
        let settings = UserSettings::load()?;
        let mut clips_client = ClipsHttpClient::new(settings.api.url)?;
        clips_client.patch(new_values, &self.clip_id).await
    }

    /// `delete` will send a call to the API to delete the clip from the DB.
    /// This method is not responsible for assigning the hosted_id to the local clip's metadata, and
    /// it's not responsible for making sure the user has a sufficient storage limit
    /// This method is basically a wrapper around ClipsHttpClient & HostedClip where we handle
    /// settings, creation of the client and sending the call
    pub async fn delete(&self) -> Result<(), WayclipError> {
        let settings = UserSettings::load()?;
        let mut clips_client = ClipsHttpClient::new(settings.api.url)?;
        clips_client.delete(&self.clip_id).await
    }
}
