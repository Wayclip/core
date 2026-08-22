use std::fmt::Display;

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{
    client::{TokensStore, authentication::AuthenticationHttpClient, users::UsersHttpClient},
    models::{
        auth::device::GetAuthDeviceResponse,
        error::WayclipError,
        users::{UsersLimitResponse, UsersResponse},
    },
    settings::UserSettings,
};

/// Basically a wrapper around UsersHttpClient. Not very useful as of right now, but could possibly
/// expand when Users will interact with I/O or whatnot
pub struct Users;

// TODO: Remove 'get' since I dont want to make it feel like its POST/GET
impl Users {
    /// Although, this method is located in Users App wrapper, it actually uses the
    /// AuthenticationHttpClient internally.
    pub async fn logout() -> Result<(), WayclipError> {
        let settings = UserSettings::load()?;
        let mut auth_client = AuthenticationHttpClient::new(settings.api.url)?;
        auth_client.logout().await
    }

    /// Although, this method is located in Users App wrapper, it actually uses the
    /// AuthenticationHttpClient internally.
    pub async fn init() -> Result<GetAuthDeviceResponse, WayclipError> {
        let settings = UserSettings::load()?;
        let mut auth_client = AuthenticationHttpClient::new(settings.api.url)?;
        auth_client.init().await
    }

    /// Although, this method is located in Users App wrapper, it actually uses the
    /// AuthenticationHttpClient internally.
    pub async fn poll(interval_s: u32, device_code: String) -> Result<TokensStore, WayclipError> {
        let settings = UserSettings::load()?;
        let mut auth_client = AuthenticationHttpClient::new(settings.api.url)?;
        auth_client.poll(interval_s, device_code).await
    }

    pub async fn get_me() -> Result<UsersResponse, WayclipError> {
        let settings = UserSettings::load()?;
        let mut users_client = UsersHttpClient::new(settings.api.url)?;
        users_client.me().await
    }

    pub async fn get_limit() -> Result<UsersLimitResponse, WayclipError> {
        let settings = UserSettings::load()?;
        let mut users_client = UsersHttpClient::new(settings.api.url)?;
        users_client.limit().await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersInfo {
    pub user: UsersResponse,
    pub limit: UsersLimitResponse,
}

impl Display for UsersInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Username: {}", self.user.username)?;
        writeln!(f, "User ID: {}", self.user.user_id)?;
        writeln!(f, "Email: {}", self.user.email)?;
        writeln!(f, "Locale: {}", self.user.locale)?;

        let created_local = self.user.created_at.with_timezone(&Local);
        writeln!(f, "Created: {}", created_local.format("%Y-%m-%d %H:%M:%S"))?;

        if let Some(verified_at) = self.user.verified_at {
            let verified_local = verified_at.with_timezone(&Local);
            writeln!(
                f,
                "Verified: {}",
                verified_local.format("%Y-%m-%d %H:%M:%S")
            )?;
        } else {
            writeln!(f, "Verified: No")?;
        }

        if let Some(ref avatar) = self.user.avatar_url {
            writeln!(f, "Avatar: {}", avatar)?;
        }

        if let Some(banned_at) = self.user.banned_at {
            let banned_local = banned_at.with_timezone(&Local);
            writeln!(f, "Banned: {}", banned_local.format("%Y-%m-%d %H:%M:%S"))?;
        }

        writeln!(f, "\nStorage & Limits:")?;
        writeln!(f, "  Tier: {}", self.limit.limit_name)?;

        if let Some(ref desc) = self.limit.limit_description {
            writeln!(f, "  Description: {}", desc)?;
        }

        let percentage = if self.limit.limit_mb > 0 {
            (self.limit.used_mb as f64 / self.limit.limit_mb as f64) * 100.0
        } else {
            0.0
        };

        writeln!(
            f,
            "  Usage: {}MB / {}MB ({:.1}%)",
            self.limit.used_mb, self.limit.limit_mb, percentage
        )?;

        Ok(())
    }
}

impl From<(UsersResponse, UsersLimitResponse)> for UsersInfo {
    fn from((user, limit): (UsersResponse, UsersLimitResponse)) -> Self {
        Self { user, limit }
    }
}

impl From<(&UsersResponse, &UsersLimitResponse)> for UsersInfo {
    fn from((user, limit): (&UsersResponse, &UsersLimitResponse)) -> Self {
        Self {
            user: user.clone(),
            limit: limit.clone(),
        }
    }
}
