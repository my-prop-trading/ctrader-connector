use error_chain::bail;
use reqwest::StatusCode;
use reqwest::Response;
use serde::de::DeserializeOwned;
use crate::rest::errors::Error;


pub async fn handle<T: DeserializeOwned>(
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