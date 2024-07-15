use serde::Serialize;
use serde_derive::Deserialize;

pub trait CtraderRequest: Serialize {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCtraderManagerTokenRequest {
    pub login: String,
    #[serde(rename = "hashedPassword")]
    pub hashed_password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCtraderManagerTokenResponse {
    #[serde(rename = "webservToken")]
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCTIDRequest {
    pub email: String,
    #[serde(rename = "brokerName")]
    pub broker_name: String,    
    #[serde(rename = "preferredLanguage")]
    pub preferred_lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCTIDResponse {
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub nickname: String,
    pub email: String,
    #[serde(rename = "preferredLanguage")]
    pub preferred_lang: String,
    #[serde(rename = "utcCreateTimestamp")]
    pub timestamp: u64,
    pub status: String, // todo: add enum
}