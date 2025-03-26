use crate::models::ManagerCreds;
use crate::utils::generate_password_hash;
use crate::webservices::endpoints::WebservicesApiEndpoint;
use crate::webservices::errors::Error;
use crate::webservices::models::{
    CreateCtidRequest, CreateCtidResponse, CreateCtraderManagerTokenRequest,
    CreateCtraderManagerTokenResponse, CreateTraderRequest,
};
use crate::webservices::{
    ClosedPositionModel, CreateTraderResponse, GetClosedPositionsRequest,
    GetOpenedPositionsRequest, GetSymbolsResponse, GetTraderGroupsResponse, GetTradersRequest,
    GetTradersResponse, LinkCtidRequest, LinkCtidResponse, OpenedPositionModel, SymbolModel,
    TraderGroupModel, TraderModel, UpdateTraderBalanceRequest, UpdateTraderBalanceResponse,
    UpdateTraderRequest,
};
use error_chain::bail;
use flurl::{FlUrl, FlUrlResponse};
use http::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait WebservicesApiConfig {
    async fn get_url(&self) -> String;
}

/// A simple yet powerful RESTful API, designed to cover the basic integration requirements for CRM
/// systems. It offers the capability to handle common CRM related tasks, such as the creation and
/// updates of users and trading accounts, and performing deposits and withdrawals to those accounts.
pub struct WebservicesApiClient<C: WebservicesApiConfig> {
    config: C,
    creds: Arc<dyn ManagerCreds + Send + Sync>,
    auth_token: std::sync::RwLock<Option<String>>,
}

impl<C: WebservicesApiConfig> WebservicesApiClient<C> {
    pub fn new(config: C, creds: Arc<dyn ManagerCreds + Send + Sync>) -> Self {
        Self {
            config,
            creds,
            auth_token: std::sync::RwLock::new(None),
        }
    }

    pub fn clear_token(&self) {
        let _ = self.auth_token.write().unwrap().take();
    }

    pub fn is_authorized(&self) -> bool {
        self.auth_token.read().unwrap().is_some()
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
        let mut token_lock = self.auth_token.write().unwrap();
        *token_lock = Some(resp.token);

        Ok(())
    }

    pub async fn create_token(&self) -> Result<CreateCtraderManagerTokenResponse, Error> {
        let request = CreateCtraderManagerTokenRequest {
            login: self.creds.get_login().await,
            hashed_password: generate_password_hash(&self.creds.get_password().await),
        };
        let endpoint = WebservicesApiEndpoint::CreateManagerToken;

        self.send_deserialized(endpoint, Some(&request)).await
    }

    pub async fn send_deserialized<R: Serialize + Debug, T: DeserializeOwned + Debug>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
    ) -> Result<T, Error> {
        self.send_flurl_deserialized(endpoint, request).await
    }

    pub async fn send<R: Serialize + Debug>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
    ) -> Result<String, Error> {
        self.send_flurl(endpoint, request).await
    }

    async fn send_flurl_deserialized<R: Serialize + Debug, T: DeserializeOwned + Debug>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
    ) -> Result<T, Error> {
        let response = self.send_flurl(endpoint, request).await?;
        let result: Result<T, _> = serde_json::from_str(&response);

        let Ok(body) = result else {
            let msg = format!(
                "Failed to deserialize: {:?}. Url: {:?} {:?}. Request: {:?}. Body: {}",
                result,
                endpoint.get_http_method(),
                String::from(&endpoint),
                request,
                response
            );
            return Err(msg.into());
        };

        Ok(body)
    }

    async fn send_flurl<R: Serialize + Debug>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
    ) -> Result<String, Error> {
        let mut request_json = None;

        if let Some(request) = request {
            let body = serde_json::to_string(request)?;
            request_json = Some(body.clone());
        }

        let request_bytes: Option<Vec<u8>> = if let Some(request) = request {
            Some(serde_json::to_string(request)?.into_bytes())
        } else {
            None
        };
        let (flurl, url) = self.build_flurl(endpoint, request).await?;
        let http_method = endpoint.get_http_method();

        let result = if http_method == Method::GET {
            flurl.get().await
        } else if http_method == Method::POST {
            flurl.post(request_bytes).await
        } else if http_method == Method::PUT {
            flurl.put(request_bytes).await
        } else if http_method == Method::PATCH {
            flurl.patch(request_bytes).await
        } else if http_method == Method::DELETE {
            flurl.delete().await
        } else {
            panic!("not implemented");
        };

        let Ok(resp) = result else {
            return Err(format!(
                "FlUrl failed to receive_body: Url: {}. Request: {:?}. {:?}",
                url,
                request_json,
                result.unwrap_err()
            )
            .into());
        };

        handle_flurl_text(resp, &request_json, &url, endpoint.get_http_method()).await
    }

    pub async fn build_flurl<R: Serialize>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
    ) -> Result<(FlUrl, String), Error> {
        let token = self.get_token_cloned();
        let base_url = self.config.get_url().await;
        let http_method = endpoint.get_http_method();

        let url = if http_method == Method::GET {
            let query_string = serde_qs::to_string(&request).expect("must be valid model");
            self.build_full_url(&base_url, &endpoint, Some(query_string), &token)
        } else {
            self.build_full_url(&base_url, &endpoint, None, &token)
        };

        let flurl = self.add_headers(FlUrl::new(&url));

        Ok((flurl, url))
    }

    fn add_headers(&self, flurl: FlUrl) -> FlUrl {
        let json_content_str = "application/json";

        flurl
            .with_header("Content-Type", json_content_str)
            .with_header("Accept", json_content_str)
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
        base_url: &str,
        endpoint: &WebservicesApiEndpoint,
        query_string: Option<String>,
        token: &Option<String>,
    ) -> String {
        let endpoint_str = String::from(endpoint);

        if let Some(token) = token {
            let token_param_name = "token";

            if let Some(query_string) = query_string {
                format!("{base_url}{endpoint_str}?{query_string}&{token_param_name}={token}")
            } else {
                format!("{base_url}{endpoint_str}?{token_param_name}={token}")
            }
        } else {
            format!("{base_url}{endpoint_str}")
        }
    }

    fn get_token_cloned(&self) -> Option<String> {
        (*self.auth_token.read().unwrap()).clone()
    }
}

async fn handle_flurl_text(
    response: FlUrlResponse,
    request_json: &Option<String>,
    request_url: &str,
    request_method: Method,
) -> Result<String, Error> {
    let status_code = StatusCode::from_u16(response.get_status_code()).unwrap();
    let result = response.receive_body().await;

    let Ok(body_bytes) = result else {
        return Err(format!("FlUrl failed to receive_body: {:?}", result.unwrap_err()).into());
    };

    let body_str = String::from_utf8(body_bytes).unwrap();

    match status_code {
        StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(body_str),
        StatusCode::INTERNAL_SERVER_ERROR => {
            bail!(format!(
                "Internal Server Error. Url: {request_method:?} {request_url}"
            ));
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            bail!(format!(
                "Service Unavailable. Url: {request_method:?} {request_url}"
            ));
        }
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            bail!(format!(
                "Unauthorized or forbidden. Url: {request_method:?} {request_url}"
            ));
        }
        StatusCode::BAD_REQUEST => {
            let error = body_str;
            bail!(format!(
                "Received bad request status. Url: {request_method:?} {request_url}. Request: {request_json:?}. Response: {error:?}"
            ));
        }
        code => {
            let error = body_str;
            bail!(format!("Received response code: {code:?}. Url: {request_method:?} {request_url}. Request: {request_json:?} Response: {error:?}"));
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
