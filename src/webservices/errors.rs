use error_chain::error_chain;
use serde_derive::{Deserialize, Serialize};

error_chain! {
    errors {
       RestError(response: String)
    }
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        ReqError(reqwest::Error);
        InvalidHeaderError(reqwest::header::InvalidHeaderValue);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        UrlParserError(url::ParseError);
        Json(serde_json::Error);
        TimestampError(std::time::SystemTimeError);
    }
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum RestApiErrorCode {
    #[strum(to_string = "TRADER_NOT_FOUND")]
    #[serde(rename = "TRADER_NOT_FOUND")]
    TraderNotFound,
}

