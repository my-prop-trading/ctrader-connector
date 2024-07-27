use chrono::{Utc};
use ctrader_connector::rest::creds::ManagerCreds;
use ctrader_connector::rest::models::CreateCtidRequest;
use ctrader_connector::rest::register_user_flow::RegisterUserFlow;
use ctrader_connector::rest::rest_client::WebservicesRestClient;
use ctrader_connector::rest::utils::generate_password_hash;
use ctrader_connector::rest::{BalanceChangeType, CreateTraderRequest, GetClosedPositionsRequest, GetTradersRequest, LinkCtidRequest, TotalMarginCalculationType, TraderAccessRights, TraderAccountType, UpdateTraderBalanceRequest};
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
    //register(&rest_client).await;
    //get_traders(&rest_client).await;
}

pub async fn get_closed_positions(rest_client: &WebservicesRestClient) {
    let request = GetClosedPositionsRequest {
        from: Default::default(),
        to: Utc::now(),
        login: None,
    };
    let resp = rest_client.get_closed_positions(&request).await;

    println!("{:?}", resp)
}

pub async fn get_traders(rest_client: &WebservicesRestClient) {
    let request = GetTradersRequest {
        from: Default::default(),
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

pub async fn register(rest_client: &WebservicesRestClient) {
    let flow = RegisterUserFlow {
        user_email: generate_test_email(),
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

    println!("{:?}", result)
}

pub async fn create_ctid(rest_client: &WebservicesRestClient) {
    let request = CreateCtidRequest {
        email: generate_test_email(),
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

fn generate_test_password_hash() -> String {
    generate_password_hash("qwerty123")
}

fn generate_test_email() -> String {
    let uuid = &Uuid::new_v4().to_string()[..6];

    format!("{}@mailinator.com", uuid)
}
