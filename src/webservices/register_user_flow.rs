use crate::utils::generate_password_hash;
use crate::webservices::api_client::{WebservicesApiClient, WebservicesApiConfig};
use crate::webservices::errors::Error;
use crate::webservices::{
    CreateCtidRequest, CreateCtidResponse, CreateTraderRequest, CreateTraderResponse,
    LinkCtidRequest, LinkCtidResponse, TotalMarginCalculationType, TraderAccessRights,
    TraderAccountType,
};

/// A wrapper for needed operations for a full user registration
#[derive(Debug, Clone)]
pub struct RegisterUserFlow {
    pub user_email: String,
    pub broker_name: String,
    pub user_password: String,
    pub deposit_currency: String,
    pub group_name: String,
    pub environment_name: String,
    pub leverage_in_cents: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub swap_free: Option<bool>,
    pub description: Option<String>,
}

impl RegisterUserFlow {
    /// 1. Create a new cTID (API call 5.1.).
    /// 2. Create a new account (API call 4.1.).
    /// 3. Link the new account to the cTID (API call 5.2).
    pub async fn execute<T: WebservicesApiConfig>(
        self,
        rest_client: &WebservicesApiClient<T>,
    ) -> Result<RegisterData, Error> {
        let create_ctid_resp = rest_client
            .create_ctid(&CreateCtidRequest {
                email: self.user_email,
                broker_name: self.broker_name.clone(),
                preferred_lang: None,
            })
            .await?;
        let password_hash = generate_password_hash(&self.user_password);

        let create_trader_resp = rest_client
            .create_trader(&CreateTraderRequest {
                access_rights: TraderAccessRights::FullAccess,
                account_type: TraderAccountType::Hedged,
                balance: 0,
                broker_name: self.broker_name.clone(),
                deposit_currency: self.deposit_currency,
                group_name: self.group_name,
                hashed_password: password_hash.clone(),
                leverage_in_cents: self.leverage_in_cents,
                total_margin_calculation_type: TotalMarginCalculationType::Max,
                contact_details: None,
                description: self.description,
                is_limited_risk: None,
                name: self.first_name,
                last_name: self.last_name,
                limited_risk_margin_calculation_strategy: None,
                max_leverage: None,
                send_own_statement: None,
                send_statement_to_broker: None,
                swap_free: self.swap_free,
            })
            .await?;

        let link_ctid_resp = rest_client
            .link_ctid(&LinkCtidRequest {
                trader_login: create_trader_resp.login,
                trader_password_hash: password_hash,
                user_id: create_ctid_resp.user_id,
                broker_name: self.broker_name,
                environment_name: self.environment_name,
                return_account_details: Some(true),
            })
            .await?;

        Ok(RegisterData {
            create_ctid_resp,
            trader: create_trader_resp,
            link_ctid_resp,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RegisterData {
    pub create_ctid_resp: CreateCtidResponse,
    pub trader: CreateTraderResponse,
    pub link_ctid_resp: LinkCtidResponse,
}
