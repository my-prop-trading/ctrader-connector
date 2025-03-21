use chrono::{TimeDelta, Utc};
use ctrader_connector::manager::api_client::{ManagerApiClient, ManagerApiConfig};
use ctrader_connector::manager::callback::ManagerApiCallbackHandler;
use ctrader_connector::manager::cs_messages_external::{
    ProtoManagerClosePositionReq, ProtoTraderListReq,
};
use ctrader_connector::manager::models::ManagerApiMessage;
use ctrader_connector::models::ManagerCreds;
use rust_extensions::Logger;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use flurl::my_tls::tokio_rustls::rustls;

#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");
    let creds = Arc::new(ExampleManagerCreds {
        password: std::env::var("CTRADER_PASSWORD").unwrap(),
        login: std::env::var("CTRADER_LOGIN").unwrap().parse().unwrap(),
    });
    let handler = Arc::new(ExampleHandler {});
    let url = std::env::var("CTRADER_MANAGER_API_URL").unwrap();
    let config = Arc::new(ExampleManagerApiConfig {
        url,
        plant_id: std::env::var("CTRADER_PLANT_ID").unwrap(),
        env_name: "demo".to_string(),
    });
    let logger = Arc::new(ConsoleLogger {});
    let client = ManagerApiClient::new(handler, config, creds, logger);
    client.connect().await.unwrap();

    //reconnect(&client).await;
    //close_position(&client).await;
    //req_trader_list(&client).await;

    loop {
        // wait for events
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

pub async fn reconnect(client: &ManagerApiClient<ExampleHandler>) {
    client.disconnect().await;
    client.connect().await.unwrap();
}

pub async fn req_trader_list(client: &ManagerApiClient<ExampleHandler>) {
    let now = Utc::now();
    let result = client
        .req_trader_list(ProtoTraderListReq {
            payload_type: None,
            from_timestamp: (now - TimeDelta::days(10)).timestamp_millis(),
            to_timestamp: now.timestamp_millis(),
            group_id: None,
            hide_ib_parameters: None,
            only_sub_accounts: None,
        })
        .await;

    println!("{:?}", result);
}

pub async fn close_position(client: &ManagerApiClient<ExampleHandler>) {
    let result = client
        .req_close_position(ProtoManagerClosePositionReq {
            payload_type: None,
            trader_id: 329768,
            position_id: 181208,
            volume: 100,
            channel: None,
        })
        .await;

    println!("{:?}", result);
}

pub struct ExampleHandler {}

#[async_trait::async_trait]
impl ManagerApiCallbackHandler for ExampleHandler {
    async fn on_connected(&self) {
        println!("connected");
    }

    async fn on_disconnected(&self) {
        println!("on_disconnected");
    }

    async fn on_message(&self, message: ManagerApiMessage) {
        println!("message: {:?}", message);
    }
}

pub struct ConsoleLogger {}

impl Logger for ConsoleLogger {
    fn write_info(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
        println!("INFO:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_warning(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
        println!("WARNING:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_error(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
        println!("ERROR:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_fatal_error(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
        println!("FATAL ERROR:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_debug_info(
        &self,
        _process: String,
        message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
        println!("DEBUG:");
        println!("{}", message);
        println!("===========================");
    }
}

pub struct ExampleManagerApiConfig {
    pub url: String,
    pub plant_id: String,
    pub env_name: String,
}

#[async_trait::async_trait]
impl ManagerApiConfig for ExampleManagerApiConfig {
    async fn get_url(&self) -> String {
        self.url.clone()
    }

    async fn get_plant_id(&self) -> String {
        self.plant_id.clone()
    }

    async fn get_env_name(&self) -> String {
        self.env_name.clone()
    }
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
