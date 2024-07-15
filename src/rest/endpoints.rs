use http::Method;

pub enum CtraderEndpoint {
    CreateManagerToken,
    CreateCTID
}

impl From<&CtraderEndpoint> for String {
    fn from(item: &CtraderEndpoint) -> Self {
        match item {
            CtraderEndpoint::CreateManagerToken => "/v2/webserv/managers/token".to_string(),
            CtraderEndpoint::CreateCTID => "/v2/ctid/create".to_string(),
        }
    }
}

impl CtraderEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            CtraderEndpoint::CreateManagerToken => Method::POST,
            CtraderEndpoint::CreateCTID => Method::POST,
        }
    }
}
