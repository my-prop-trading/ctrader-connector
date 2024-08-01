use crate::creds::ManagerCreds;
use crate::webservices::endpoints::WebservicesApiEndpoint;
use crate::webservices::errors::Error;
use crate::webservices::models::{
    CreateCtidRequest, CreateCtidResponse, CreateCtraderManagerTokenRequest,
    CreateCtraderManagerTokenResponse, CreateTraderRequest,
};
use crate::utils::generate_password_hash;
use crate::webservices::{
    ClosedPositionModel, CreateTraderResponse, GetClosedPositionsRequest,
    GetOpenedPositionsRequest, GetSymbolsResponse, GetTraderGroupsResponse, GetTradersRequest,
    GetTradersResponse, LinkCtidRequest, LinkCtidResponse, OpenedPositionModel, SymbolModel,
    TraderGroupModel, TraderModel, UpdateTraderBalanceRequest, UpdateTraderBalanceResponse,
    UpdateTraderRequest,
};
use error_chain::bail;
use http::{Method, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use tokio::sync::RwLock;

/// A simple yet powerful RESTful API, designed to cover the basic integration requirements for CRM
/// systems. It offers the capability to handle common CRM related tasks, such as the creation and
/// updates of users and trading accounts, and performing deposits and withdrawals to those accounts.
pub struct WebservicesClient {
    url: String,
    inner_client: reqwest::Client,
    creds: ManagerCreds,
    auth_token: RwLock<Option<String>>,
}

impl WebservicesClient {
    pub fn new(url: impl Into<String>, creds: ManagerCreds) -> Self {
        Self {
            url: url.into(),
            inner_client: reqwest::Client::new(),
            creds,
            auth_token: RwLock::new(None),
        }
    }

    pub async fn is_authorized(&self) -> bool {
        self.auth_token.read().await.is_some()
    }

    /// Gets the list of all available symbols on the server.
    pub async fn get_symbols(&self) -> Result<Vec<SymbolModel>, Error> {
        let request: Option<&String> = None;
        let endpoint = WebservicesApiEndpoint::GetSymbols;
        let resp: GetSymbolsResponse = self.send_deserialized(endpoint, request).await?;

        Ok(resp.items)
    }

    /// Gets a list of all trader groups.
    pub async fn get_trader_groups(&self) -> Result<Vec<TraderGroupModel>, Error> {
        let request: Option<&String> = None;
        let endpoint = WebservicesApiEndpoint::GetTraderGroups;
        let resp: GetTraderGroupsResponse = self.send_deserialized(endpoint, request).await?;

        Ok(resp.items)
    }

    pub async fn get_traders(
        &self,
        request: &GetTradersRequest,
    ) -> Result<Vec<TraderModel>, Error> {
        let endpoint = WebservicesApiEndpoint::GetTraders;
        let resp: GetTradersResponse = self.send_deserialized(endpoint, Some(request)).await?;

        Ok(resp.items)
    }

    pub async fn get_trader(&self, login: i64) -> Result<TraderModel, Error> {
        let request: Option<&String> = None;
        let endpoint = WebservicesApiEndpoint::GetTrader(login);

        self.send_deserialized(endpoint, request).await
    }

    /// Gets either a list of all closed positions or a list of closed positions originated
    /// by a specific trader entity (if the login parameter is specified).
    /// Note 1:
    /// The difference between the timestamps specified in the from and to parameters must be equal to two days or less.
    /// Note 2:
    /// Each row in the output represents one specific result of closing a position,
    /// be it wholly or partially. As a result, you may see cases where positionId is repeated
    /// across several rows. This is intended behavior as it represents positions that were
    /// closed via multiple deals.
    pub async fn get_closed_positions(
        &self,
        request: &GetClosedPositionsRequest,
    ) -> Result<Vec<ClosedPositionModel>, Error> {
        let endpoint = WebservicesApiEndpoint::GetClosedPositions;
        let data = self.send(endpoint, Some(request)).await?;

        parse_positions(&data)
    }

    /// Gets either a list of all open positions or a list of open positions originated by
    /// a specific trader entity (if the login parameter is specified).
    pub async fn get_opened_positions(
        &self,
        request: &GetOpenedPositionsRequest,
    ) -> Result<Vec<OpenedPositionModel>, Error> {
        let endpoint = WebservicesApiEndpoint::GetOpenedPositions;
        let data = self.send(endpoint, Some(request)).await?;

        parse_positions(&data)
    }

    /// Changes the balance of a trader entity (including allocating/removing credit).
    pub async fn update_trader_balance(
        &self,
        request: &UpdateTraderBalanceRequest,
    ) -> Result<UpdateTraderBalanceResponse, Error> {
        let endpoint = WebservicesApiEndpoint::UpdateTraderBalance(request.login);
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Updates a trader entity.
    pub async fn update_trader(
        &self,
        login: i64,
        request: &UpdateTraderRequest,
    ) -> Result<(), Error> {
        let endpoint = WebservicesApiEndpoint::UpdateTrader(login);
        let _ = self.send(endpoint, Some(request)).await?;

        Ok(())
    }

    /// Links a trader entity to a user entity.
    pub async fn link_ctid(&self, request: &LinkCtidRequest) -> Result<LinkCtidResponse, Error> {
        let endpoint = WebservicesApiEndpoint::LinkCtid;
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Creates a new trader (e.g. account)entity.
    pub async fn create_trader(
        &self,
        request: &CreateTraderRequest,
    ) -> Result<CreateTraderResponse, Error> {
        let endpoint = WebservicesApiEndpoint::CreateTrader;
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Creates a new user entity. The cTID is used to authorize end users in the trading application(s) of their choice
    pub async fn create_ctid(
        &self,
        request: &CreateCtidRequest,
    ) -> Result<CreateCtidResponse, Error> {
        let endpoint = WebservicesApiEndpoint::CreateCtid;
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Creates a token and stores it internally for the next requests
    pub async fn authorize(&self) -> Result<(), Error> {
        let resp = self.create_token().await?;
        let mut token_lock = self.auth_token.write().await;
        *token_lock = Some(resp.token);

        Ok(())
    }

    pub async fn create_token(&self) -> Result<CreateCtraderManagerTokenResponse, Error> {
        let request = CreateCtraderManagerTokenRequest {
            login: self.creds.login.clone(),
            hashed_password: generate_password_hash(&self.creds.password),
        };
        let endpoint = WebservicesApiEndpoint::CreateManagerToken;

        self.send_deserialized(endpoint, Some(&request)).await
    }

    pub async fn send_deserialized<R: Serialize, T: DeserializeOwned + Debug>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
    ) -> Result<T, Error> {
        let token = &*self.auth_token.read().await;
        let (builder, url, request) = self.get_builder(endpoint, request, token)?;
        let response = builder.send().await;

        handle_json(response?, request, &url).await
    }

    pub async fn send<R: Serialize>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
    ) -> Result<String, Error> {
        let token = &*self.auth_token.read().await;
        let (builder, url, request) = self.get_builder(endpoint, request, token)?;
        let response = builder.send().await;

        handle_text(response?, &request, &url).await
    }

    fn get_builder<R: Serialize>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
        token: &Option<String>,
    ) -> Result<(RequestBuilder, String, Option<String>), Error> {
        let headers = self.build_headers();
        let http_method = endpoint.get_http_method();
        let mut request_json = None;

        let url = if http_method == Method::GET {
            let query_string = serde_qs::to_string(&request).expect("must be valid model");
            self.build_full_url(&endpoint, Some(query_string), token)
        } else {
            self.build_full_url(&endpoint, None, token)
        };

        let mut builder = self.inner_client.request(http_method, &url);

        if let Some(request) = request {
            let body = serde_json::to_string(request)?;
            request_json = Some(body.clone());
            builder = builder.body(body);
        }

        Ok((builder.headers(headers), url, request_json))
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
        token: &Option<String>,
    ) -> String {
        let url = &self.url;
        let endpoint_str = String::from(endpoint);

        if let Some(token) = token {
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

async fn handle_json<T: DeserializeOwned + Debug>(
    response: Response,
    request_json: Option<String>,
    request_url: &str,
) -> Result<T, Error> {
    let text = handle_text(response, &request_json, request_url).await?;
    let result: Result<T, _> = serde_json::from_str(&text);

    let Ok(body) = result else {
        bail!(
            "Failed to deserialize body. Url: {}.  Error: {:?}. Body: {}",
            request_url,
            result.unwrap_err(),
            text
        );
    };

    Ok(body)
}

async fn handle_text(
    response: Response,
    request_json: &Option<String>,
    request_url: &str,
) -> Result<String, Error> {
    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
            let result: Result<String, _> = response.text().await;

            let Ok(text) = result else {
                bail!(format!("Failed to read response body. Url {request_url}"));
            };

            Ok(text)
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            bail!(format!("Internal Server Error. Url: {request_url}"));
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            bail!(format!("Service Unavailable. Url: {request_url}"));
        }
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            bail!(format!("Unauthorized or forbidden. Url: {request_url}"));
        }
        StatusCode::BAD_REQUEST => {
            let error = response.text().await?;
            bail!(format!(
                "Received bad request status. Url: {request_url}. Request: {request_json:?}. Response: {error:?}"
            ));
        }
        code => {
            let error = response.text().await?;
            bail!(format!("Received response code: {code:?}. Url: {request_url}. Request: {request_json:?} Response: {error:?}"));
        }
    }
}

pub fn parse_positions<T: DeserializeOwned + Debug>(data: &str) -> Result<Vec<T>, Error> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(data.as_bytes());
    let mut positions = Vec::with_capacity(20);

    for result in reader.deserialize() {
        let result: Result<T, _> = result;

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
    use crate::webservices::api_client::parse_positions;
    use crate::webservices::{ClosedPositionModel, OpenedPositionModel};

    #[test]
    fn parses_closed_positions() {
        let data = r#"login,positionId,dealId,openTimestamp,closeTimestamp,entryPrice,closePrice,direction,volume,symbol,commission,swap,pnl,depositConversionRate,usdConversionRate,bookType,stake,spreadBetting
9013206,6101,4690813,2018-03-19T13:44:21.224,2020-01-02T09:01:53.613,0.69999,0.70132,BUY,1000.00,AUD/USD,0.01,142.74,76.17,0.96911,0.70132,BOOK_B,0.00,false
9013197,13313,5690189,2018-08-30T12:15:29.154,2020-01-02T09:01:54.748,1.32315,1.32214,BUY,1000.00,GBPUSD,0.00,25.83,44.79,0.96911,1.32214,BOOK_B,0.00,false"#;

        let result: Result<Vec<ClosedPositionModel>, _> = parse_positions(data);

        println!("{:?}", result);

        let positions = result.unwrap();
        assert_eq!(positions.len(), 2);
    }

    #[test]
    fn parses_opened_positions() {
        let data = r#"login,positionId,openTimestamp,entryPrice,direction,volume,symbol,commission,swap,bookType,stake,spreadBetting,usedMargin
9013206,4325443,2020-08-28T05:35:37.682,1.18657,BUY,1000.00,EURUSD,1.69,2.54,BOOK_B,0.00,false,0.01
9013197,4325446,2020-08-28T05:35:38.015,1.18656,BUY,1000.00,EURUSD,1.69,2.54,BOOK_B,0.00,false,0.01"#;

        let result: Result<Vec<OpenedPositionModel>, _> = parse_positions(data);

        println!("{:?}", result);

        let positions = result.unwrap();
        assert_eq!(positions.len(), 2);
    }
}
