use http::Method;

pub enum CtraderEndpoint {
    CreateManagerToken,
    CreateCtid,
    CreateTrader,
    LinkCtid,
}

impl From<&CtraderEndpoint> for String {
    fn from(item: &CtraderEndpoint) -> Self {
        let api_version = "v2";

        match item {
            CtraderEndpoint::CreateManagerToken => {
                format!("/{}/webserv/managers/token", api_version)
            }
            CtraderEndpoint::CreateCtid => format!("/{}/ctid/create", api_version),
            CtraderEndpoint::CreateTrader => format!("/{}/webserv/traders", api_version),
            CtraderEndpoint::LinkCtid => format!("/{}//ctid/link", api_version),
        }
    }
}

impl CtraderEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            CtraderEndpoint::CreateManagerToken => Method::POST,
            CtraderEndpoint::CreateCtid => Method::POST,
            CtraderEndpoint::CreateTrader => Method::POST,
            CtraderEndpoint::LinkCtid => Method::POST,
        }
    }
}
