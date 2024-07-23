use crate::rest::errors::Error;
use crate::rest::rest_client::WebServicesRestClient;
use crate::rest::utils::generate_password_hash;
use crate::rest::{
    CreateCtidRequest, CreateTraderRequest, LinkCtidRequest, TotalMarginCalculationType,
    TraderAccessRights, TraderAccountType,
};

#[derive(Debug, Clone)]
pub struct RegisterUserFlow {
    pub user_email: String,
    pub broker_name: String,
    pub user_password: String,
    pub deposit_currency: String,
    pub group_name: String,
    pub environment_name: String,
}

impl RegisterUserFlow {
    pub async fn execute(
        self,
        rest_client: &WebServicesRestClient,
    ) -> Result<RegisterUserInfo, Error> {
        // 1. Create a new cTID (API call 5.1.).
        // 2. Create a new account (API call 4.1.).
        // 3. Link the new account to the cTID (API call 5.2).

        let create_ctid_resp = rest_client
            .create_ctid(CreateCtidRequest {
                email: self.user_email,
                broker_name: self.broker_name.clone(),
                preferred_lang: None,
            })
            .await?;
        let password_hash = generate_password_hash(&self.user_password);

        let create_trader_resp = rest_client
            .create_trader(CreateTraderRequest {
                access_rights: TraderAccessRights::FullAccess,
                account_type: TraderAccountType::Hedged,
                balance: 0,
                broker_name: self.broker_name.clone(),
                deposit_currency: self.deposit_currency,
                group_name: self.group_name,
                hashed_password: password_hash.clone(),
                leverage_in_cents: 0,
                total_margin_calculation_type: TotalMarginCalculationType::Max,
                contact_details: None,
                description: None,
                is_limited_risk: None,
                last_name: None,
                limited_risk_margin_calculation_strategy: None,
                max_leverage: None,
                name: None,
                send_own_statement: None,
                send_statement_to_broker: None,
                swap_free: None,
            })
            .await?;

        let link_ctid_resp = rest_client
            .link_ctid(LinkCtidRequest {
                trader_login: create_trader_resp.login,
                trader_password_hash: password_hash,
                user_id: create_ctid_resp.user_id,
                broker_name: self.broker_name,
                environment_name: self.environment_name,
                return_account_details: Some(true),
            })
            .await?;

        Ok(RegisterUserInfo {
            trader_login: create_trader_resp.login,
            user_id: create_ctid_resp.user_id,
            account_id: link_ctid_resp
                .ctid_trader_account_id
                .expect("return_account_details is true"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RegisterUserInfo {
    pub trader_login: i32,
    pub user_id: i32,
    pub account_id: i32,
}
