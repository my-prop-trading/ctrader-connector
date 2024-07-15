use error_chain::error_chain;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ErrorContent {
    pub errors: HashMap<String, String>,
}

error_chain! {
    errors {
       GenericError(response: ErrorContent)
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
