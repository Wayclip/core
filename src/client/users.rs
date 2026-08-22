use crate::{
    client::WayclipClient,
    models::{
        error::WayclipError,
        users::{UsersLimitResponse, UsersResponse},
    },
};
use reqwest::Method;

pub struct UsersHttpClient {
    client: WayclipClient,
}

impl UsersHttpClient {
    pub fn new(api_url: url::Url) -> Result<Self, WayclipError> {
        let client = WayclipClient::new(api_url)?;
        Ok(Self { client })
    }

    pub async fn limit(&mut self) -> Result<UsersLimitResponse, WayclipError> {
        let response = self
            .client
            .with_credentials()
            .await?
            .send_call(Method::GET, "users/me/limit")
            .await?;
        response.into_inner()
    }

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
