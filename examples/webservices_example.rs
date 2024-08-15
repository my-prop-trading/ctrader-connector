use chrono::{Duration, TimeDelta, Utc};
use ctrader_connector::models::ManagerCreds;
use ctrader_connector::utils::generate_password_hash;
use ctrader_connector::webservices::api_client::{WebservicesApiClient, WebservicesApiConfig};
use ctrader_connector::webservices::errors::Error;
use ctrader_connector::webservices::models::CreateCtidRequest;
use ctrader_connector::webservices::register_user_flow::{RegisterData, RegisterUserFlow};
use ctrader_connector::webservices::{
    BalanceChangeType, CreateTraderRequest, GetClosedPositionsRequest, GetOpenedPositionsRequest,
    GetTradersRequest, LinkCtidRequest, TotalMarginCalculationType, TraderAccessRights,
    TraderAccountType, UpdateTraderBalanceRequest, UpdateTraderRequest,
};
use futures_util::future::try_join_all;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use std::ops::Sub;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::sleep;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let creds = Arc::new(ExampleManagerCreds {
        password: std::env::var("CTRADER_PASSWORD").unwrap(),
        login: std::env::var("CTRADER_LOGIN").unwrap().parse().unwrap(),
    });
    let config = ExampleWebservicesApiConfig {
        url: std::env::var("CTRADER_URL").unwrap(),
    };

    let rest_client = WebservicesApiClient::new(config, creds);
    rest_client.authorize().await.unwrap();
    //let data = register(&rest_client).await.unwrap();
    //make_deposit(&rest_client, data.trader.login, 1000.0).await;
    //get_opened_positions(&rest_client, Some(3238431)).await;
    //get_closed_positions(&rest_client, Some(3238505)).await;
    //update_group(&rest_client, 3238431, "enabled_accounts").await;
    //update_access_rights(&rest_client, 3238431, TraderAccessRights::FullAccess).await;
    get_trader(&rest_client, 3238507).await;
    //get_groups(&rest_client).await;
    //get_symbols(&rest_client).await;
    //get_traders(&rest_client).await;
    //get_closed_parallel(&rest_client, 3238431, 300).await;
}

pub async fn get_symbols(rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>) {
    let resp = rest_client.get_symbols().await;

    println!("{:?}", resp)
}

pub async fn get_groups(rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>) {
    let resp = rest_client.get_trader_groups().await;

    println!("{:?}", resp)
}

pub async fn get_trader(
    rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>,
    login: i64,
) {
    let resp = rest_client.get_trader(login).await;

    println!("{:?}", resp);
    println!(
        "profit: {}",
        resp.as_ref().unwrap().equity - resp.as_ref().unwrap().balance
    );
}

pub async fn update_group(
    rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>,
    login: i64,
    group_name: impl Into<String>,
) {
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

pub async fn update_access_rights(
    rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>,
    login: i64,
    access_rights: TraderAccessRights,
) {
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

pub async fn get_opened_positions(
    rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>,
    login: Option<i64>,
) {
    let request = GetOpenedPositionsRequest { login };
    let resp = rest_client.get_opened_positions(&request).await;

    println!("{:?}", resp)
}

pub async fn get_closed_positions(
    rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>,
    login: Option<i64>,
) {
    let date = DateTimeAsMicroseconds::from_str("2024-07-30T17:47:50.545Z")
        .unwrap()
        .to_chrono_utc();
    let request = GetClosedPositionsRequest {
        from: date + TimeDelta::microseconds(1),
        to: Utc::now(),
        login,
    };
    let resp = rest_client.get_closed_positions(&request).await;

    println!("{:?}", resp)
}

pub async fn get_traders(rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>) {
    let request = GetTradersRequest {
        from: Utc::now().sub(TimeDelta::try_days(600).unwrap()),
        to: Utc::now(),
        group_id: None,
    };
    let resp = rest_client.get_traders(&request).await;

    println!("{:?}", resp)
}

pub async fn deposit(rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>) {
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

pub async fn register(
    rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>,
) -> Result<RegisterData, Error> {
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
        description: None,
    };
    let result = flow.execute(rest_client).await;

    println!("{:?}", result);

    result
}

pub async fn create_ctid(rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>) {
    let request = CreateCtidRequest {
        email: get_test_email(),
        broker_name: std::env::var("CTRADER_BROKER_NAME").unwrap(),
        preferred_lang: None,
    };
    let resp = rest_client.create_ctid(&request).await;

    println!("{:?}", resp)
}

pub async fn create_trader(rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>) {
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

pub async fn link_ctid(rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>) {
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

pub async fn make_deposit(
    rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>,
    login: i64,
    precise_amount: f64,
) {
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
    "maksim.g@mailinator.com".to_string()
}

pub struct MockRestClient;

impl MockRestClient {
    pub async fn get_closed_positions(&self, request: &MockRequest) -> Result<(), ()> {
        sleep(core::time::Duration::from_millis(1000)).await;
        println!("Request: from {} to {}", request.from, request.to);
        Ok(())
    }
}

pub struct MockRequest {
    pub from: chrono::DateTime<Utc>,
    pub to: chrono::DateTime<Utc>,
    pub login: Option<u64>,
}

pub async fn get_closed_parallel(
    rest_client: &WebservicesApiClient<ExampleWebservicesApiConfig>,
    login: i64,
    days: i64,
) {
    let days_period = 1;
    let num_requests = days / days_period;
    let max_concurrent_requests = 10;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_requests));

    let futures = (0..num_requests).map(|i| {
        let semaphore = Arc::clone(&semaphore);

        let client = &rest_client;
        let now = Utc::now();
        let from = now - Duration::try_days((i + 1) * days_period).unwrap();
        let to = now - Duration::try_days(i * days_period).unwrap();
        async move {
            let _permit = semaphore
                .acquire()
                .await
                .expect("Semaphore wasn't been closed");

            client
                .get_closed_positions(&GetClosedPositionsRequest {
                    from,
                    to,
                    login: Some(login),
                })
                .await
        }
    });

    let result: Result<_, _> = try_join_all(futures).await;
    println!("{:?}", result);
}

pub struct ExampleManagerCreds {
    pub login: i64,
    pub password: String,
}

#[async_trait::async_trait]
impl ManagerCreds for ExampleManagerCreds {
    async fn get_password(&self) -> String {
        self.password.clone()
    }

    async fn get_login(&self) -> i64 {
        self.login
    }
}

pub struct ExampleWebservicesApiConfig {
    pub url: String,
}

#[async_trait::async_trait]
impl WebservicesApiConfig for ExampleWebservicesApiConfig {
    async fn get_url(&self) -> String {
        self.url.clone()
    }
}
