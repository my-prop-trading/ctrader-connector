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
