use ctrader_connector::manager::api_client::{ManagerApiClient, ManagerApiConfig};
use ctrader_connector::manager::callback::ManagerApiCallbackHandler;
use ctrader_connector::manager::models::ManagerApiMessage;
use ctrader_connector::models::ManagerCreds;
use rust_extensions::Logger;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let creds = ManagerCreds {
        password: std::env::var("CTRADER_PASSWORD").unwrap(),
        login: std::env::var("CTRADER_LOGIN").unwrap().parse().unwrap(),
    };
    let handler = Arc::new(ExampleHandler {});
    let url = std::env::var("CTRADER_MANAGER_API_URL").unwrap();
    let config = Arc::new(ManagerApiConfig {
        url,
        creds,
        plant_id: std::env::var("CTRADER_PLANT_ID").unwrap(),
        env_name: "demo".to_string(),
    });
    let logger = Arc::new(ConsoleLogger {});
    let client = ManagerApiClient::new(handler, config, logger);
    client.connect().await;

    loop { // wait for events
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
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

    async fn on_message(&self, event: ManagerApiMessage) {
        println!("event: {:?}", event);
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
