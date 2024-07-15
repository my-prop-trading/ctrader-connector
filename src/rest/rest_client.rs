use crate::rest::creds::CTraderCreds;
use crate::rest::endpoints::CtraderEndpoint;
use crate::rest::errors::Error;
use crate::rest::models::{
    CreateCTIDRequest, CreateCTIDResponse, CreateCtraderManagerTokenRequest,
    CreateCtraderManagerTokenResponse, CtraderRequest,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CtraderRestClient {
    url: String,
    inner_client: reqwest::Client,
    creds: CTraderCreds,
    current_token: Option<String>,
}

impl CtraderRestClient {
    pub fn new(host: &str, port: usize, creds: CTraderCreds) -> Self {
        Self {
            url: format!("https://{}:{}", host, port),
            inner_client: reqwest::Client::new(),
            creds,
            current_token: None,
        }
    }

    pub async fn create_ctid(
        &self,
        request: CreateCTIDRequest,
    ) -> Result<CreateCTIDResponse, Error> {
        let url: String = format!(
            "{}{}?token={}",
            self.url,
            String::from(&CtraderEndpoint::CreateManagerToken),
            self.current_token.clone().unwrap_or("".to_string())
        );
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
        let password_digest = md5::compute(self.creds.password.as_bytes());
        let request = CreateCtraderManagerTokenRequest {
            login: self.creds.login.clone(),
            hashed_password: format!("{:x}", password_digest),
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
            "content-type",
            HeaderValue::from_str("application/json").unwrap(),
        );

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
