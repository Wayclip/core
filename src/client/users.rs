use crate::{
    client::WayclipClient,
    models::{
        error::WayclipError,
        users::{UsersLimitResponse, UsersResponse},
    },
};
use reqwest::Method;

/// This struct is a submodule of the WayclipClient, responsible for mananging actions related to
/// users
pub struct UsersHttpClient {
    client: WayclipClient,
}

impl UsersHttpClient {
    /// Create new client
    pub fn new(api_url: url::Url) -> Result<Self, WayclipError> {
        let client = WayclipClient::new(api_url)?;
        Ok(Self { client })
    }

    /// Sends a call to get the storage limit
    pub async fn limit(&mut self) -> Result<UsersLimitResponse, WayclipError> {
        let response = self
            .client
            .with_credentials()
            .await?
            .send_call(Method::GET, "users/me/limit")
            .await?;
        response.into_inner()
    }

    /// Sends a call to get information about the currently logged in user
    pub async fn me(&mut self) -> Result<UsersResponse, WayclipError> {
        let response = self
            .client
            .with_credentials()
            .await?
            .send_call(Method::GET, "/users/me")
            .await?;
        response.into_inner()
    }
}
