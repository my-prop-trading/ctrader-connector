use ctrader_connector::rest::creds::CTraderCreds;
use ctrader_connector::rest::models::CreateCTIDRequest;
use ctrader_connector::rest::rest_client::CtraderRestClient;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let creds = CTraderCreds {
        password: std::env::var("CTRADER_PASSWORD").unwrap(),
        login: std::env::var("CTRADER_LOGIN").unwrap(),
    };
    let host = std::env::var("CTRADER_HOST").unwrap();
    let port = std::env::var("CTRADER_PORT").unwrap().parse().unwrap();

    let mut rest_client = CtraderRestClient::new(&host, port, creds);
    rest_client.authorize().await.unwrap();
}

pub async fn create_ctid(rest_client: &CtraderRestClient) {
    let uuid = &Uuid::new_v4().to_string()[..6];
    let request = CreateCTIDRequest {
        email: format!("{}@mailinator.com", uuid),
        broker_name: std::env::var("CTRADER_BROKER_NAME").unwrap(),
        preferred_lang: None,
    };
    let resp = rest_client.create_ctid(request).await;

    println!("{:?}", resp)
}
