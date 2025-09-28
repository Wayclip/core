use crate::ClipJsonData;
use crate::HashMap;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::env;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub github_id: Option<i64>,
    pub username: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub is_banned: bool,
    pub subscription: UserSubscription,
    pub two_factor_enabled: bool,
    #[serde(skip)]
    pub two_factor_secret: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub role: UserRole,
    pub last_login_at: Option<DateTime<Utc>>,
    pub last_login_ip: Option<String>,
    pub security_stamp: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    tier_id: Uuid,
    name: String,
    max_storage_bytes: u64,
    stripe_price_id: Option<String>,
}

impl TierConfig {
    pub fn from_json_value(json_value: &serde_json::Value) -> Self {
        let storage_str = json_value["max_storage"].as_str().unwrap_or("5GB");
        let max_storage_bytes =
            Self::parse_size(storage_str).expect("Invalid max_storage format in TIERS_JSON");

        TierConfig {
            tier_id: Uuid::default(),
            name: json_value["name"].as_str().unwrap_or("Unnamed").to_string(),
            max_storage_bytes,
            stripe_price_id: json_value["stripe_price_id"]
                .as_str()
                .map(|s| s.to_string()),
        }
    }

    pub fn parse_size(size_str: &str) -> Result<u64, String> {
        let s = size_str.trim().to_uppercase();
        let (value_str, unit) = s.split_at(s.trim_end_matches(|c: char| c.is_alphabetic()).len());

        let value = value_str
            .parse::<u64>()
            .map_err(|_| format!("Invalid number in size string: '{size_str}'"))?;

        const KB: u64 = 1024;
        const MB: u64 = 1024 * KB;
        const GB: u64 = 1024 * MB;
        const TB: u64 = 1024 * GB;

        match unit {
            "B" | "" => Ok(value),
            "KB" => Ok(value * KB),
            "MB" => Ok(value * MB),
            "GB" => Ok(value * GB),
            "TB" => Ok(value * TB),
            _ => Err(format!("Unknown size unit '{unit}' in string '{size_str}'")),
        }
    }
}

pub fn load_tiers() -> HashMap<Uuid, TierConfig> {
    let json = std::env::var("TIERS_JSON").unwrap_or_else(|_| "[]".to_string());
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Invalid TIERS_JSON format");

    parsed
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|v| {
            let tier = TierConfig::from_json_value(v);
            (tier.tier_id, tier)
        })
        .collect()
}

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, sqlx::Type, Display, EnumString,
)]
#[sqlx(type_name = "credential_provider", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CredentialProvider {
    Email,
    GitHub,
    Google,
    Discord,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, sqlx::Type, Display, EnumString,
)]
#[sqlx(type_name = "subscription_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Trialing,
    PastDue,
    Incomplete,
    Canceled,
    Unpaid,
    Disputed,
}

#[derive(Debug, Serialize, FromRow, Deserialize, Clone)]
pub struct UserSubscription {
    pub tier_id: Uuid,
    pub stripe_subscription_id: Option<String>,
    pub status: SubscriptionStatus,
    pub cancel_at_period_end: bool,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubUser {
    pub id: i64,
    pub login: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUser {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Clip {
    pub id: Uuid,
    pub user_id: Uuid,
    pub file_name: String,
    pub file_size: i64,
    pub public_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    #[serde(flatten)]
    pub user: User,
    pub storage_used: i64,
    pub storage_limit: i64,
    pub clip_count: i64,
    pub connected_accounts: Vec<CredentialProvider>,
}

#[derive(Debug, Serialize, FromRow, Deserialize, Clone)]
pub struct HostedClipInfo {
    pub id: Uuid,
    pub file_name: String,
    pub file_size: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UnifiedClipData {
    pub name: String,
    pub full_filename: String,
    pub local_path: Option<String>,
    pub local_data: Option<ClipJsonData>,
    pub created_at: DateTime<Local>,
    pub is_hosted: bool,
    pub hosted_id: Option<Uuid>,
}
