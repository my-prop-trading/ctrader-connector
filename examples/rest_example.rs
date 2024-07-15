use ctrader_connector::rest::creds::CTraderCreds;
use ctrader_connector::rest::rest_client::CtraderRestClient;

#[tokio::main]
async fn main() {
    let creds = CTraderCreds {
        password: std::env::var("PASSWORD").unwrap(),
        login: std::env::var("LOGIN").unwrap(),
    };
    let host = std::env::var("HOST").unwrap();
    let port = std::env::var("PORT").unwrap().parse().unwrap();

    let mut rest_client = CtraderRestClient::new(&host, port, creds);
    rest_client.authorize().await.unwrap();
}
