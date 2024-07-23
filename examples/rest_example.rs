use ctrader_connector::rest::creds::ManagerCreds;
use ctrader_connector::rest::models::CreateCtidRequest;
use ctrader_connector::rest::rest_client::WebServicesRestClient;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let creds = ManagerCreds {
        password: std::env::var("CTRADER_PASSWORD").unwrap(),
        login: std::env::var("CTRADER_LOGIN").unwrap().parse().unwrap(),
    };
    let url = std::env::var("CTRADER_URL").unwrap();
    println!("{}", url);

    let mut rest_client = WebServicesRestClient::new(url, creds);
    rest_client.authorize().await.unwrap();
}

pub async fn create_ctid(rest_client: &WebServicesRestClient) {
    let uuid = &Uuid::new_v4().to_string()[..6];
    let request = CreateCtidRequest {
        email: format!("{}@mailinator.com", uuid),
        broker_name: std::env::var("CTRADER_BROKER_NAME").unwrap(),
        preferred_lang: None,
    };
    let resp = rest_client.create_ctid(request).await;

    println!("{:?}", resp)
}
