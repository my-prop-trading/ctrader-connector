use crate::rest::creds::ManagerCreds;
use crate::rest::endpoints::CtraderEndpoint;
use crate::rest::errors::Error;
use crate::rest::models::{
    CreateCtidRequest, CreateCtidResponse, CreateCtraderManagerTokenRequest,
    CreateCtraderManagerTokenResponse, CreateTraderRequest, CreateTraderResponse, CtraderRequest,
};
use crate::rest::{LinkCtidRequest, LinkCtidResponse};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use crate::rest::utils::generate_password_hash;

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

    fn generate_endpoint_url(&self, endpoint: &CtraderEndpoint) -> String {
        let Some(token) = self.current_token.as_ref() else {
            panic!("Must be authorized at this point");
        };

        format!("{}{}?token={}", self.url, String::from(endpoint), token)
    }

    /// Links a trader entity to a user entity.
    pub async fn link_ctid(&self, request: LinkCtidRequest) -> Result<LinkCtidResponse, Error> {
        let url = self.generate_endpoint_url(&CtraderEndpoint::LinkCtid);
        let headers = self.build_headers();
        let request_json = serde_json::to_string(&request)?;

        let response = self
            .inner_client
            .post(&url)
            .body(request_json.clone())
            .headers(headers)
            .send()
            .await;

        crate::rest::response_handler::handle(response?, Some(request_json), &url).await
    }

    /// Creates a new trader (e.g. account)entity.
    pub async fn create_trader(
        &self,
        request: CreateTraderRequest,
    ) -> Result<CreateTraderResponse, Error> {
        let url = self.generate_endpoint_url(&CtraderEndpoint::CreateTrader);
        let headers = self.build_headers();
        let request_json = serde_json::to_string(&request)?;

        let response = self
            .inner_client
            .post(&url)
            .body(request_json.clone())
            .headers(headers)
            .send()
            .await;

        crate::rest::response_handler::handle(response?, Some(request_json), &url).await
    }

    /// Creates a new user entity. The cTID is used to authorize end users in the trading application(s) of their choice
    pub async fn create_ctid(
        &self,
        request: CreateCtidRequest,
    ) -> Result<CreateCtidResponse, Error> {
        let url = self.generate_endpoint_url(&CtraderEndpoint::CreateCtid);
        let headers = self.build_headers();
        let request_json = serde_json::to_string(&request)?;

        let response = self
            .inner_client
            .post(&url)
            .body(request_json.clone())
            .headers(headers)
            .send()
            .await;

        crate::rest::response_handler::handle(response?, Some(request_json), &url).await
    }

    pub async fn authorize(&mut self) -> Result<(), Error> {
        let url: String = format!(
            "{}{}",
            self.url,
            String::from(&CtraderEndpoint::CreateManagerToken)
        );
        let headers = self.build_headers();
        let request = CreateCtraderManagerTokenRequest {
            login: self.creds.login.clone(),
            hashed_password: generate_password_hash(&self.creds.password),
        };
        let request_json = serde_json::to_string(&request)?;

        let response = self
            .inner_client
            .post(&url)
            .body(request_json.clone())
            .headers(headers)
            .send()
            .await;
        let response: CreateCtraderManagerTokenResponse =
            crate::rest::response_handler::handle(response?, Some(request_json), &url).await?;

        self.current_token = Some(response.token);

        Ok(())
    }

    pub async fn post<R: CtraderRequest, T: DeserializeOwned>(
        &self,
        endpoint: CtraderEndpoint,
        request: R,
    ) -> Result<T, Error> {
        let url: String = format!("{}{}", self.url, String::from(&endpoint));
        let headers = self.build_headers();
        let client = &self.inner_client;
        let request_json = serde_json::to_string(&request)?;

        let response = client
            .post(&url)
            .body(request_json.clone())
            .headers(headers)
            .send()
            .await;

        crate::rest::response_handler::handle(response?, Some(request_json), &url).await
    }

    fn build_headers(&self) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        custom_headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());

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
