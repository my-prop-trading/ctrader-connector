use crate::rest::creds::ManagerCreds;
use crate::rest::endpoints::WebservicesApiEndpoint;
use crate::rest::errors::Error;
use crate::rest::models::{
    CreateCtidRequest, CreateCtidResponse, CreateCtraderManagerTokenRequest,
    CreateCtraderManagerTokenResponse, CreateTraderRequest,
};
use crate::rest::utils::generate_password_hash;
use crate::rest::{
    ClosedPositionModel, GetClosedPositionsRequestQuery, GetTradersRequestQuery,
    GetTradersResponse, LinkCtidRequest, LinkCtidResponse, TraderModel, UpdateTraderBalanceRequest,
    UpdateTraderBalanceResponse, UpdateTraderRequest,
};
use error_chain::bail;
use http::{Method, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// A simple yet powerful RESTful API, designed to cover the basic integration requirements for CRM
/// systems. It offers the capability to handle common CRM related tasks, such as the creation and
/// updates of users and trading accounts, and performing deposits and withdrawals to those accounts.
#[derive(Clone)]
pub struct WebservicesRestClient {
    url: String,
    inner_client: reqwest::Client,
    creds: ManagerCreds,
    current_token: Option<String>,
}

impl WebservicesRestClient {
    pub fn new(url: impl Into<String>, creds: ManagerCreds) -> Self {
        Self {
            url: url.into(),
            inner_client: reqwest::Client::new(),
            creds,
            current_token: None,
        }
    }

    pub async fn get_traders(
        &self,
        request: &GetTradersRequestQuery,
    ) -> Result<Vec<TraderModel>, Error> {
        let endpoint = WebservicesApiEndpoint::GetTraders;
        let resp: GetTradersResponse = self.send(endpoint, request).await?;
        
        Ok(resp.trader)
    }

    pub async fn get_closed_positions(
        &self,
        request: &GetClosedPositionsRequestQuery,
    ) -> Result<Vec<ClosedPositionModel>, Error> {
        let endpoint = WebservicesApiEndpoint::GetClosedPositions;
        let data: String = self.send(endpoint, request).await?;

        parse_closed_positions(&data)
    }

    /// Changes the balance of a trader entity (including allocating/removing credit).
    pub async fn update_trader_balance(
        &self,
        request: &UpdateTraderBalanceRequest,
    ) -> Result<UpdateTraderBalanceResponse, Error> {
        let endpoint = WebservicesApiEndpoint::UpdateTraderBalance(request.login.to_string());
        self.send(endpoint, request).await
    }

    /// Updates a trader entity.
    pub async fn update_trader(
        &self,
        login: i64,
        request: &UpdateTraderRequest,
    ) -> Result<(), Error> {
        let endpoint = WebservicesApiEndpoint::UpdateTrader(login.to_string());
        self.send(endpoint, request).await
    }

    /// Links a trader entity to a user entity.
    pub async fn link_ctid(&self, request: &LinkCtidRequest) -> Result<LinkCtidResponse, Error> {
        let endpoint = WebservicesApiEndpoint::LinkCtid;
        self.send(endpoint, request).await
    }

    /// Creates a new trader (e.g. account)entity.
    pub async fn create_trader(&self, request: &CreateTraderRequest) -> Result<TraderModel, Error> {
        let endpoint = WebservicesApiEndpoint::CreateTrader;
        self.send(endpoint, request).await
    }

    /// Creates a new user entity. The cTID is used to authorize end users in the trading application(s) of their choice
    pub async fn create_ctid(
        &self,
        request: &CreateCtidRequest,
    ) -> Result<CreateCtidResponse, Error> {
        let endpoint = WebservicesApiEndpoint::CreateCtid;
        self.send(endpoint, request).await
    }

    pub async fn authorize(&mut self) -> Result<(), Error> {
        let request = CreateCtraderManagerTokenRequest {
            login: self.creds.login.clone(),
            hashed_password: generate_password_hash(&self.creds.password),
        };
        let endpoint = WebservicesApiEndpoint::CreateManagerToken;

        let response: CreateCtraderManagerTokenResponse = self.send(endpoint, &request).await?;

        self.current_token = Some(response.token);

        Ok(())
    }

    pub async fn send<R: Serialize, T: DeserializeOwned>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: &R,
    ) -> Result<T, Error> {
        let headers = self.build_headers();
        let http_method = endpoint.get_http_method();
        let mut request_json = None;
        let url: String;

        let builder = if http_method == Method::GET {
            let query_string = serde_qs::to_string(&request).expect("must be valid model");
            url = self.build_full_url(&endpoint, Some(query_string));
            self.inner_client.request(http_method, &url)
        } else {
            let body = serde_json::to_string(request)?;
            request_json = Some(body.clone());
            url = self.build_full_url(&endpoint, None);
            self.inner_client.request(http_method, &url).body(body)
        };

        let response = builder.headers(headers).send().await;

        handle(response?, request_json, &url).await
    }

    fn build_headers(&self) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();
        let json_content_str = "application/json";

        custom_headers.insert(
            "Content-Type",
            HeaderValue::from_str(json_content_str).unwrap(),
        );
        custom_headers.insert("Accept", HeaderValue::from_str(json_content_str).unwrap());

        custom_headers
    }

    pub fn build_query_string(&self, params: Vec<(&str, &str)>) -> String {
        let mut query_string = String::new();

        for (key, value) in params {
            let param = format!("{key}={value}&");
            query_string.push_str(&param);
        }

        query_string.pop(); // remove last & symbol

        query_string
    }

    fn build_full_url(
        &self,
        endpoint: &WebservicesApiEndpoint,
        query_string: Option<String>,
    ) -> String {
        let url = &self.url;
        let endpoint_str = String::from(endpoint);

        if let Some(token) = self.current_token.as_ref() {
            let token_param_name = "token";

            if let Some(query_string) = query_string {
                format!("{url}{endpoint_str}?{query_string}&{token_param_name}={token}")
            } else {
                format!("{url}{endpoint_str}?{token_param_name}={token}")
            }
        } else {
            format!("{url}{endpoint_str}")
        }
    }
}

async fn handle<T: DeserializeOwned>(
    response: Response,
    request_json: Option<String>,
    request_url: &str,
) -> Result<T, Error> {
    match response.status() {
        StatusCode::OK => {
            let json: Result<String, _> = response.text().await;
            let Ok(json) = json else {
                bail!("Failed to read response body. Url {}", request_url);
            };

            let body: Result<T, _> = serde_json::from_str(&json);
            if let Err(err) = body {
                bail!(
                    "Url {}. Failed to deserialize body {:?}: {}",
                    request_url,
                    err,
                    json
                );
            }

            Ok(body.unwrap())
        }
        StatusCode::CREATED => {
            let json: Result<String, _> = response.text().await;
            let Ok(json) = json else {
                bail!("Failed to read response body");
            };
            let body: Result<T, _> = serde_json::from_str(&json);
            if let Err(err) = body {
                bail!("Failed to deserialize body {:?}: {}", err, json);
            }

            Ok(body.unwrap())
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            bail!("Internal Server Error {}", request_url,);
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            bail!("Service Unavailable {}", request_url,);
        }
        StatusCode::UNAUTHORIZED => {
            bail!("Unauthorized {}", request_url);
        }
        StatusCode::BAD_REQUEST => {
            let error = response.text().await?;
            bail!(format!(
                "Received bad request status. Url: {}. Request: {:?}. Response: {:?}",
                request_url, request_json, error
            ));
        }
        s => {
            let error = response.text().await?;

            bail!(format!("Received response code: {s:?} error: {error:?}"));
        }
    }
}

pub fn parse_closed_positions(data: &str) -> Result<Vec<ClosedPositionModel>, Error> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(data.as_bytes());
    let mut positions = Vec::with_capacity(20);

    for result in reader.deserialize() {
        let result: Result<ClosedPositionModel, _> = result;

        let Ok(position) = result else {
            let msg = format!("Failed to parse: {:?}. Resp: {data}", result.unwrap_err());
            return Err(msg.into());
        };

        positions.push(position)
    }

    Ok(positions)
}

#[cfg(test)]
mod tests {
    use crate::rest::rest_client::parse_closed_positions;

    #[test]
    fn parses_closed_positions() {
        let data = r#"login,positionId,dealId,openTimestamp,closeTimestamp,entryPrice,closePrice,direction,volume,symbol,commission,swap,pnl,depositConversionRate,usdConversionRate,bookType,stake,spreadBetting
9013206,6101,4690813,2018-03-19T13:44:21.224,2020-01-02T09:01:53.613,0.69999,0.70132,BUY,1000.00,AUD/USD,0.01,142.74,76.17,0.96911,0.70132,BOOK_B,0.00,false
9013197,13313,5690189,2018-08-30T12:15:29.154,2020-01-02T09:01:54.748,1.32315,1.32214,BUY,1000.00,GBPUSD,0.00,25.83,44.79,0.96911,1.32214,BOOK_B,0.00,false"#;

        let result = parse_closed_positions(data);

        println!("{:?}", result);

        let positions = result.unwrap();
        assert_eq!(positions.len(), 2);
    }
}
