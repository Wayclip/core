use crate::ClipJsonData;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub github_id: Option<i64>,
    pub username: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub tier: SubscriptionTier,
    pub created_at: DateTime<Utc>,
    pub is_banned: bool,
    pub stripe_customer_id: Option<String>,
    pub two_factor_enabled: bool,
    #[serde(skip)]
    pub two_factor_secret: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
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
#[sqlx(type_name = "subscription_tier", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SubscriptionTier {
    Free,
    #[strum(serialize = "tier1")]
    Tier1,
    #[strum(serialize = "tier2")]
    Tier2,
    #[strum(serialize = "tier3")]
    Tier3,
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
    Canceled,
    Unpaid,
}

#[derive(Debug, Serialize, FromRow, Deserialize, Clone)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stripe_subscription_id: String,
    pub stripe_price_id: String,
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

#[derive(Debug)]
pub struct UnifiedClipData {
    pub name: String,
    pub full_filename: String,

    pub local_path: Option<String>,
    pub local_data: Option<ClipJsonData>,
    pub created_at: DateTime<Local>,

    pub is_hosted: bool,
    pub hosted_id: Option<uuid::Uuid>,
}
