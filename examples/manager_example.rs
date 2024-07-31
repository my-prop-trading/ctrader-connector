use std::collections::HashMap;
use ctrader_connector::manager::api_client::{ManagerApiClient, ManagerApiConfig};
use ctrader_connector::manager::callback::ManagerApiCallbackHandler;
use ctrader_connector::manager::serialization::ManagerApiEvent;
use rust_extensions::Logger;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let handler = Arc::new(ExampleHandler {});
    let url = std::env::var("CTRADER_MANAGER_API_URL").unwrap();
    //let parsed_url = url::Url::parse(&url).unwrap();
    let mut splits = url.split(':').map(|v| v.to_string());
    let config = ManagerApiConfig {
        server_name: splits.next().unwrap(),
        host_port: url,
    };
    println!("{:?}", config);
    let logger = Arc::new(ConsoleLogger {});
    let client = ManagerApiClient::new(handler, config, logger);
    let result = client.connect().await;
    println!("{:?}", result);

    loop {
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

    async fn on_event(&self, event: ManagerApiEvent) {
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
        println!("{}", message);
    }

    fn write_warning(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
        println!("{}", message);
    }

    fn write_error(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
        println!("{}", message);
    }

    fn write_fatal_error(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
        println!("{}", message);
    }

    fn write_debug_info(
        &self,
        _process: String,
        message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
        println!("{}", message);
    }
}
