use serde::{Deserialize, Serialize};

/// Type returned by API after initialising the authentication proceedure
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct GetAuthDeviceResponse {
    /// The code the CLI will use to fetch data if user has logged in. Very long
    pub device_code: String,
    /// A simple code that user will type in once logged in
    pub user_code: String,
    /// The interval, in seconds, with which the CLI will poll if user completed authentication
    pub interval: u32,
}

/// The request when polling if user has completed authentication
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PostAuthDevicePollRequest {
    /// The code the CLI will use to fetch data if user has logged in. Very long
    pub device_code: String,
}

/// The response recieved when user completed authenetication
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PostAuthDevicePollResponse {
    /// The user_id of the user
    pub user_id: String,
}
