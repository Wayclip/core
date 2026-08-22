use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct GetAuthDeviceResponse {
    pub device_code: String,
    pub user_code: String,
    pub interval: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PostAuthDevicePollRequest {
    pub device_code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PostAuthDevicePollResponse {
    pub user_id: String,
}
