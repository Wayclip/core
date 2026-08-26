//! src/client will be responsbile for making any API/Network calls, such as Authentication calls,
//! Clips handling, Quering data. This is done to generalise the code base, so that same client is
//! used, which will handle any network requests same way

use crate::{app::os_keyring::OsKeyring, models::error::WayclipError};
use cookie_rs::Cookie;
use reqwest::{
    Method, StatusCode,
    header::{COOKIE, GetAll, HeaderValue, SET_COOKIE},
    multipart::Form,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{fmt::Debug, sync::Arc};

/// We standardise the naming convention of the two cookies that are used for authentication.
/// One of them being `jwt_token`
pub const JWT_TOKEN: &str = "jwt_token";
/// We standardise the naming convention of the two cookies that are used for authentication.
/// The other one being `refresh_token`
pub const REFRESH_TOKEN: &str = "refresh_token";

/// A submodule, a wrapper of such, which uses the `WayclipClient` to accomplish a goal assosciated
/// with providing authentication
pub mod authentication;
/// A submodule, a wrapper of such, which uses the `WayclipClient` to accomplish a goal assosciated
/// with managing hosted clips
pub mod clips;
/// A submodule, a wrapper of such, which uses the `WayclipClient` to accomplish a goal assosciated
/// with managing users (/me)
pub mod users;

/// This struct holds the information needed to one or more calls to the API. It holds the endpoint
/// Url, `reqwest` Client and other information.
/// The `tokens`, `body` and `multipart_builder` are populated by the submodule when it seems fit.
/// This allows us to make various types of requests, no matter the method, url, what body to be
/// passed or even multipart data
#[derive(Clone)]
pub struct WayclipClient {
    api_endpoint: url::Url,
    http_client: reqwest::Client,
    // These 3 will be populated later on
    tokens: Option<TokensStore>,
    body: Option<serde_json::Value>,
    multipart_builder: Option<Arc<dyn Fn() -> Form + Send + Sync>>,
}

impl WayclipClient {
    /// General method to construct a WayclipClient
    pub fn new(api_url: url::Url) -> Result<Self, WayclipError> {
        let tld = api_url
            .host_str()
            .ok_or_else(|| WayclipError::NotFound("No host found in URL".into()))?
            .split(".")
            .last()
            .ok_or_else(|| WayclipError::NotFound("No last element".into()))?;

        let http_client = reqwest::ClientBuilder::new()
            .user_agent("Wayclip")
            // TODO: Unsafe
            .danger_accept_invalid_certs(tld == "test")
            .build()?;

        Ok(Self {
            api_endpoint: api_url,
            http_client,
            tokens: None,
            body: None,
            multipart_builder: None,
        })
    }

    // On every response, we will pass all headers here
    // This will get any returned tokens from the headers
    // Compare them against existing ones & if they are not matching & not empty
    // Then it will replace them + update
    async fn update_credentials(
        &mut self,
        headers: GetAll<'_, HeaderValue>,
    ) -> Result<(), WayclipError> {
        let os_keyring = OsKeyring;
        let fetched_tokens = TokensStore::build_from_headers(headers)?;

        if &fetched_tokens != self.tokens.as_ref().unwrap_or(&TokensStore::default())
            && !fetched_tokens.jwt_token.is_empty()
            && !fetched_tokens.refresh_token.is_empty()
        {
            self.tokens = Some(fetched_tokens.clone());
            os_keyring.store::<TokensStore>(fetched_tokens).await?;
        }

        Ok(())
    }

    async fn clear_credentials(&mut self) -> Result<(), WayclipError> {
        let os_keyring = OsKeyring;
        os_keyring.clear().await?;
        self.tokens = None;
        Ok(())
    }

    /// A method for the submodules to ensure that with the next call, we will use our credentials.
    /// In addition, this stores the extracted credentials inside the Client itself.
    pub async fn with_credentials(&mut self) -> Result<&mut Self, WayclipError> {
        let os_keyring = OsKeyring;
        let tokens = os_keyring
            .get::<TokensStore>()
            .await?
            .ok_or_else(|| WayclipError::CLI("No tokens stored".into()))?;

        self.tokens = Some(tokens);
        Ok(self)
    }

    /// A method for the submodules to ensure that with the next call, we will use the provided
    /// body, which has to be serialisable.
    /// In addition, this stores the body inside the Client itself.
    pub async fn with_body<B>(&mut self, body: &B) -> Result<&mut Self, WayclipError>
    where
        B: Serialize + Send + 'static,
    {
        let value = serde_json::to_value(body)?;
        self.body = Some(value);
        Ok(self)
    }

    /// A method for the submodules to ensure that with the next call, we will use the provided
    /// multipart_builder. The multipart builder is neccesary to take ownership of the multipart
    /// when needed.
    /// In addition, this stores the multipart_builder inside the Client itself.
    pub async fn with_multipart(
        &mut self,
        multipart_builder: Arc<dyn Fn() -> Form + Send + Sync>,
    ) -> &mut Self {
        self.multipart_builder = Some(multipart_builder);
        self
    }

    /// The main method which calls the selected endpoint with the correct method.
    /// If the client contains, body, tokens, or a multipart_builder, they are automatically added
    /// to this request.
    /// If the request succeeds, we return a `WayclipResponse<R>`. This allows us to pass in the
    /// return code, as well as data
    /// If the request fails, but the erros was due to authentication, that *most* likely suggests
    /// that the `refresh_token` has expired. This prompts us clear the credentials
    /// If the request fails on other terms, we just return an error
    pub async fn send_call<R>(
        &mut self,
        method: Method,
        path: &str,
    ) -> Result<WayclipResponse<R>, WayclipError>
    where
        R: DeserializeOwned + Send + 'static,
    {
        let url = self.api_endpoint.join(path)?;
        let mut request = self.http_client.request(method, url);

        // Attach all params (if they exist)
        if let Some(ref tokens) = self.tokens {
            request = request.header(COOKIE, tokens.to_cookie_string());
        }
        if let Some(ref multipart_builder) = self.multipart_builder {
            request = request.multipart(multipart_builder());
        }
        if let Some(ref body) = self.body {
            request = request.json(body);
        }

        let response = request.send().await?;
        let headers = response.headers().get_all(SET_COOKIE);
        self.update_credentials(headers).await?;

        match response.status().is_success() {
            true => Ok(WayclipResponse::try_from_reqwest(response).await?),
            false if response.status() == StatusCode::UNAUTHORIZED => {
                self.clear_credentials().await?;
                Err(WayclipError::Validation("Session expired".into()))
            }
            false => Err(WayclipError::Api(
                format!(
                    "Server error: {:?}",
                    response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Failed to read body".into())
                )
                .into(),
            )),
        }
    }
}

/// The struct that holds both jwt and refresh token, which is what gets stored inside the keyring.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct TokensStore {
    jwt_token: String,
    refresh_token: String,
}

impl TokensStore {
    fn build_from_headers(headers: GetAll<'_, HeaderValue>) -> Result<Self, WayclipError> {
        let mut data = TokensStore::default();
        for cookie_val in headers.iter().filter_map(|c| c.to_str().ok()) {
            let cookie = Cookie::parse(cookie_val)
                .map_err(|e| WayclipError::Validation(e.to_string().into()))?;

            let value = cookie.value().trim().to_string();
            match cookie.name() {
                JWT_TOKEN => data.jwt_token = value,
                REFRESH_TOKEN => data.refresh_token = value,
                _ => (),
            };
        }
        Ok(data)
    }

    fn to_cookie_string(&self) -> String {
        format!(
            "jwt_token={}; refresh_token={}",
            self.jwt_token, self.refresh_token
        )
    }
}

/// Very useful if outer function wants to know exact return status
/// Gives us the response code (which in this case will always be of 2xx type, as well as data)
#[derive(Debug)]
pub enum WayclipResponse<R: DeserializeOwned + Send + 'static> {
    /// 200
    Ok(R),
    /// 201
    Created(R),
    /// 202
    Accepted(Option<R>),
    /// 203
    NonAuthoritativeInformation(R),
    /// 204
    NoContent,
    /// 205
    ResetContent,
    /// 206
    PartialContent(R),
    /// 207
    MultiStatus(R),
    /// 208
    AlreadyReported(R),
    /// 226
    ImUsed(R),
}

impl<R: DeserializeOwned + Send + 'static> WayclipResponse<R> {
    /// Parses the WayclipResponse, to extract the inner data
    pub fn into_inner(self) -> Result<R, WayclipError> {
        match self {
            WayclipResponse::Ok(r)
            | WayclipResponse::Created(r)
            | WayclipResponse::NonAuthoritativeInformation(r)
            | WayclipResponse::PartialContent(r)
            | WayclipResponse::MultiStatus(r)
            | WayclipResponse::AlreadyReported(r)
            | WayclipResponse::ImUsed(r)
            | WayclipResponse::Accepted(Some(r)) => Ok(r),
            WayclipResponse::Accepted(None) => Err(WayclipError::Api(
                "Received 202 Accepted without body".into(),
            )),
            WayclipResponse::NoContent => Err(WayclipError::Api("Received 204 No Content".into())),
            WayclipResponse::ResetContent => {
                Err(WayclipError::Api("Received 205 Reset Content".into()))
            }
        }
    }

    /// A method to construct the WayclipResponse from a `reqwest::Response`
    pub async fn try_from_reqwest(value: reqwest::Response) -> Result<Self, WayclipError> {
        let status = value.status();

        Ok(match status {
            StatusCode::OK => Self::Ok(value.json::<R>().await?),
            StatusCode::CREATED => Self::Created(value.json::<R>().await?),
            StatusCode::ACCEPTED => {
                let bytes = value.bytes().await?;
                if bytes.is_empty() || bytes.iter().all(|b| b.is_ascii_whitespace()) {
                    Self::Accepted(None)
                } else {
                    let body: R = serde_json::from_slice(&bytes)?;
                    Self::Accepted(Some(body))
                }
            }
            StatusCode::NO_CONTENT => Self::NoContent,
            StatusCode::RESET_CONTENT => Self::ResetContent,
            StatusCode::NON_AUTHORITATIVE_INFORMATION => {
                Self::NonAuthoritativeInformation(value.json::<R>().await?)
            }
            StatusCode::PARTIAL_CONTENT => Self::PartialContent(value.json::<R>().await?),
            StatusCode::MULTI_STATUS => Self::MultiStatus(value.json::<R>().await?),
            StatusCode::ALREADY_REPORTED => Self::AlreadyReported(value.json::<R>().await?),
            StatusCode::IM_USED => Self::ImUsed(value.json::<R>().await?),
            _ => {
                return Err(WayclipError::Api(
                    "Status was not 2xx success, but error_for_status did not trigger an error"
                        .into(),
                ));
            }
        })
    }
}
