use crate::app::os_keyring::OsKeyring;
use crate::client::{TokensStore, WayclipClient, WayclipResponse};
use crate::models::auth::device::{
    GetAuthDeviceResponse, PostAuthDevicePollRequest, PostAuthDevicePollResponse,
};
use crate::models::error::WayclipError;
use reqwest::Method;
use std::time::Duration;
use tokio::time::interval;

pub const POLL_DATA_TIMEOUT_S: i64 = 120;

pub struct AuthenticationHttpClient {
    client: WayclipClient,
}

impl AuthenticationHttpClient {
    pub fn new(api_url: url::Url) -> Result<Self, WayclipError> {
        let client = WayclipClient::new(api_url)?;
        Ok(Self { client })
    }

    pub async fn init(&mut self) -> Result<GetAuthDeviceResponse, WayclipError> {
        let res = self.client.send_call(Method::GET, "/auth/device").await?;

        res.into_inner()
    }

    pub async fn poll(
        &mut self,
        interval_s: u32,
        device_code: String,
    ) -> Result<TokensStore, WayclipError> {
        let mut timer = interval(Duration::from_secs(interval_s as u64));
        let start = chrono::Utc::now();

        loop {
            let now = chrono::Utc::now();
            if now.timestamp() - start.timestamp() > POLL_DATA_TIMEOUT_S {
                return Err(WayclipError::Api(
                    format!("Timeout of {POLL_DATA_TIMEOUT_S}s reached").into(),
                ));
            }

            timer.tick().await;

            let body = PostAuthDevicePollRequest {
                device_code: device_code.clone(),
            };

            let response: WayclipResponse<PostAuthDevicePollResponse> = self
                .client
                .with_body(&body)
                .await?
                .send_call(Method::POST, "auth/device/poll")
                .await?;

            match response {
                WayclipResponse::Ok(_) | WayclipResponse::Accepted(_)
                    if let Some(tokens) = self.client.tokens.clone() =>
                {
                    return Ok(tokens);
                }
                _ => continue,
            }
        }
    }

    pub async fn logout(&mut self) -> Result<(), WayclipError> {
        match self
            .client
            .with_credentials()
            .await?
            .send_call::<()>(Method::POST, "/auth/logout")
            .await
        {
            Ok(_) => {
                let os_keyring = OsKeyring;
                os_keyring.clear().await?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
