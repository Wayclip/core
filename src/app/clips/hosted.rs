use crate::{
    client::clips::{ClipsHttpClient, PatchClipField},
    models::{clips::hosted::HostedClip, error::WayclipError},
    settings::UserSettings,
};

/// Basically a wrapper around ClipsHttpClient & HostedClip (a.k.a ClipsResponse)
impl HostedClip {
    pub async fn patch(&self, new_values: Vec<PatchClipField>) -> Result<(), WayclipError> {
        let settings = UserSettings::load()?;
        let mut clips_client = ClipsHttpClient::new(settings.api.url)?;
        clips_client.patch(new_values, &self.clip_id).await
    }

    pub async fn delete(&self) -> Result<(), WayclipError> {
        let settings = UserSettings::load()?;
        let mut clips_client = ClipsHttpClient::new(settings.api.url)?;
        clips_client.delete(&self.clip_id).await
    }
}
