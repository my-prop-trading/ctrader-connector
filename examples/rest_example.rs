use ctrader_connector::rest::rest_client::CtraderRestClient;

#[tokio::main]
async fn main() {
    let _rest_client = CtraderRestClient::new("test", 80);
}
