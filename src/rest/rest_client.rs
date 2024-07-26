use crate::rest::creds::ManagerCreds;
use crate::rest::endpoints::WebservicesApiEndpoint;
use crate::rest::errors::Error;
use crate::rest::models::{
    CreateCtidRequest, CreateCtidResponse, CreateCtraderManagerTokenRequest,
    CreateCtraderManagerTokenResponse, CreateTraderRequest, CreateTraderResponse,
};
use crate::rest::utils::generate_password_hash;
use crate::rest::{
    LinkCtidRequest, LinkCtidResponse, UpdateTraderBalanceRequest, UpdateTraderBalanceResponse,
    UpdateTraderRequest,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

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

    fn generate_full_url(&self, endpoint: &WebservicesApiEndpoint) -> String {
        let Some(token) = self.current_token.as_ref() else {
            return format!("{}{}", self.url, String::from(endpoint));
        };

        format!("{}{}?token={}", self.url, String::from(endpoint), token)
    }

    /// Links a trader entity to a user entity.
    pub async fn link_ctid(&self, request: &LinkCtidRequest) -> Result<LinkCtidResponse, Error> {
        let endpoint = WebservicesApiEndpoint::LinkCtid;
        self.send(endpoint, request).await
    }

    /// Creates a new trader (e.g. account)entity.
    pub async fn create_trader(
        &self,
        request: &CreateTraderRequest,
    ) -> Result<CreateTraderResponse, Error> {
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
        let url = self.generate_full_url(&endpoint);
        let headers = self.build_headers();
        let request_json = serde_json::to_string(request)?;

        let response = self
            .inner_client
            .request(endpoint.get_http_method(), &url)
            .body(request_json.clone())
            .headers(headers)
            .send()
            .await;

        crate::rest::response_handler::handle(response?, Some(request_json), &url).await
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

    pub fn build_query(&self, parameters: HashMap<String, String>) -> String {
        let mut request = String::new();
        for (key, value) in parameters {
            let param = format!("{key}={value}&");
            request.push_str(param.as_ref());
        }
        request.pop();

        request
    }
}
