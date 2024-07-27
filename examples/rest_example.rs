use std::ops::{Sub};
use chrono::{TimeDelta, Utc};
use ctrader_connector::rest::creds::ManagerCreds;
use ctrader_connector::rest::errors::Error;
use ctrader_connector::rest::models::CreateCtidRequest;
use ctrader_connector::rest::register_user_flow::{RegisterData, RegisterUserFlow};
use ctrader_connector::rest::rest_client::WebservicesRestClient;
use ctrader_connector::rest::utils::generate_password_hash;
use ctrader_connector::rest::{BalanceChangeType, CreateTraderRequest, GetClosedPositionsRequest, GetOpenedPositionsRequest, GetTradersRequest, LinkCtidRequest, TotalMarginCalculationType, TraderAccessRights, TraderAccountType, UpdateTraderBalanceRequest, UpdateTraderRequest};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let creds = ManagerCreds {
        password: std::env::var("CTRADER_PASSWORD").unwrap(),
        login: std::env::var("CTRADER_LOGIN").unwrap().parse().unwrap(),
    };
    let url = std::env::var("CTRADER_URL").unwrap();

    let mut rest_client = WebservicesRestClient::new(url, creds);
    rest_client.authorize().await.unwrap();
    //let data = register(&rest_client).await.unwrap();
    //make_deposit(&rest_client, data.trader.login, 1000.0).await;
    //get_opened_positions(&rest_client, Some(3238431)).await;
    //get_closed_positions(&rest_client, Some(3238431)).await;
    //update_group(&rest_client, 3238431, "enabled_accounts").await;
    //update_access_rights(&rest_client, 3238431, TraderAccessRights::FullAccess).await;
    get_trader(&rest_client, 3238431).await;
    //get_groups(&rest_client).await;
    //get_symbols(&rest_client).await;
    get_traders(&rest_client).await;
}

pub async fn get_symbols(rest_client: &WebservicesRestClient) {
    let resp = rest_client.get_symbols().await;

    println!("{:?}", resp)
}

pub async fn get_groups(rest_client: &WebservicesRestClient) {
    let resp = rest_client.get_trader_groups().await;

    println!("{:?}", resp)
}

pub async fn get_trader(rest_client: &WebservicesRestClient, login: i64) {
    let resp = rest_client.get_trader(login).await;

    println!("{:?}", resp)
}

pub async fn update_group(rest_client: &WebservicesRestClient, login: i64, group_name: impl Into<String>) {
    let request = UpdateTraderRequest {
        access_rights: None,
        account_type: None,
        broker_name: None,
        deposit_currency: None,
        group_name: Some(group_name.into()),
        hashed_password: None,
        leverage_in_cents: None,
        total_margin_calculation_type: None,
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
    };
    let resp = rest_client.update_trader(login, &request).await;

    println!("{:?}", resp)
}

pub async fn update_access_rights(rest_client: &WebservicesRestClient, login: i64, access_rights: TraderAccessRights) {
    let request = UpdateTraderRequest {
        access_rights: Some(access_rights),
        account_type: None,
        broker_name: None,
        deposit_currency: None,
        group_name: None,
        hashed_password: None,
        leverage_in_cents: None,
        total_margin_calculation_type: None,
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
    };
    let resp = rest_client.update_trader(login, &request).await;

    println!("{:?}", resp)
}

pub async fn get_opened_positions(rest_client: &WebservicesRestClient, login: Option<i64>) {
    let request = GetOpenedPositionsRequest {
        login,
    };
    let resp = rest_client.get_opened_positions(&request).await;

    println!("{:?}", resp)
}

pub async fn get_closed_positions(rest_client: &WebservicesRestClient, login: Option<i64>) {
    let request = GetClosedPositionsRequest {
        from: Utc::now().sub(TimeDelta::try_days(2).unwrap()),
        to: Utc::now(),
        login,
    };
    let resp = rest_client.get_closed_positions(&request).await;

    println!("{:?}", resp)
}

pub async fn get_traders(rest_client: &WebservicesRestClient) {
    let request = GetTradersRequest {
        from: Utc::now().sub(TimeDelta::try_days(20).unwrap()),
        to: Utc::now(),
        group_id: None,
    };
    let resp = rest_client.get_traders(&request).await;

    println!("{:?}", resp)
}

pub async fn deposit(rest_client: &WebservicesRestClient) {
    let result = rest_client
        .update_trader_balance(&UpdateTraderBalanceRequest {
            comment: None,
            external_id: None,
            external_note: None,
            login: 3238402,
            precise_amount: 1.0,
            source: None,
            change_type: BalanceChangeType::Deposit,
        })
        .await;

    println!("{:?}", result)
}

pub async fn register(rest_client: &WebservicesRestClient) -> Result<RegisterData, Error> {
    let flow = RegisterUserFlow {
        user_email: get_test_email(),
        broker_name: std::env::var("CTRADER_BROKER_NAME").unwrap(),
        user_password: "qwerty123".to_string(),
        deposit_currency: "USD".to_string(),
        group_name: "default".to_string(),
        environment_name: "demo".to_string(),
        leverage_in_cents: 1000,
        first_name: None,
        last_name: None,
        swap_free: None,
    };
    let result = flow.execute(rest_client).await;

    println!("{:?}", result);

    result
}

pub async fn create_ctid(rest_client: &WebservicesRestClient) {
    let request = CreateCtidRequest {
        email: get_test_email(),
        broker_name: std::env::var("CTRADER_BROKER_NAME").unwrap(),
        preferred_lang: None,
    };
    let resp = rest_client.create_ctid(&request).await;

    println!("{:?}", resp)
}

pub async fn create_trader(rest_client: &WebservicesRestClient) {
    let request = CreateTraderRequest {
        access_rights: TraderAccessRights::FullAccess,
        account_type: TraderAccountType::Hedged,
        balance: 0,
        broker_name: std::env::var("CTRADER_BROKER_NAME").unwrap(),
        deposit_currency: "USD".to_string(),
        group_name: "default".to_string(),
        hashed_password: generate_test_password_hash(),
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
    };
    let resp = rest_client.create_trader(&request).await;

    println!("{:?}", resp)
}

pub async fn link_ctid(rest_client: &WebservicesRestClient) {
    let request = LinkCtidRequest {
        trader_login: 0,
        trader_password_hash: generate_test_password_hash(),
        user_id: 0,
        broker_name: std::env::var("CTRADER_BROKER_NAME").unwrap(),
        environment_name: "demo".to_string(),
        return_account_details: Some(true),
    };
    let resp = rest_client.link_ctid(&request).await;

    println!("{:?}", resp)
}

pub async fn make_deposit(rest_client: &WebservicesRestClient, login: i64, precise_amount: f64) {
    let request = UpdateTraderBalanceRequest {
        comment: None,
        external_id: None,
        external_note: None,
        login,
        precise_amount,
        source: None,
        change_type: BalanceChangeType::Deposit,
    };
    let resp = rest_client.update_trader_balance(&request).await;

    println!("{:?}", resp)
}

fn generate_test_password_hash() -> String {
    generate_password_hash("qwerty123")
}

pub fn generate_test_email() -> String {
    let uuid = &Uuid::new_v4().to_string()[..6];

    format!("{}@mailinator.com", uuid)
}

pub fn get_test_email() -> String {
    "1a351423c@mailinator.com".to_string()
}
