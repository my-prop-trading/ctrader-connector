use crate::rest::endpoints::CtraderEndpoint;
use crate::rest::errors::Error;
use crate::rest::models::CtraderRequest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CtraderRestClient {
    url: String,
    inner_client: reqwest::Client,
}

impl CtraderRestClient {
    pub fn new(host: &str, port: usize) -> Self {
        Self {
            url: format!("https://{}:{}", host, port),
            inner_client: reqwest::Client::new(),
        }
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
