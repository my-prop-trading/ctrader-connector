use http::Method;

pub enum CtraderEndpoint {
    CreateManagerToken,
}

impl From<&CtraderEndpoint> for String {
    fn from(item: &CtraderEndpoint) -> Self {
        match item {
            CtraderEndpoint::CreateManagerToken => "/webserv/managers/token".to_string(),
        }
    }
}

impl CtraderEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            CtraderEndpoint::CreateManagerToken => Method::POST,
        }
    }
}
