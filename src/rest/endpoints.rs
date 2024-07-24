use http::Method;

pub enum CtraderEndpoint {
    CreateManagerToken,
    CreateCtid,
    CreateTrader,
    LinkCtid,
    UpdateTrader(String),
}

impl From<&CtraderEndpoint> for String {
    fn from(item: &CtraderEndpoint) -> Self {
        let api_version = "v2";

        match item {
            CtraderEndpoint::CreateManagerToken => {
                format!("/{}/webserv/managers/token", api_version)
            }
            CtraderEndpoint::CreateCtid => format!("/{api_version}/ctid/create"),
            CtraderEndpoint::CreateTrader => format!("/{api_version}/webserv/traders"),
            CtraderEndpoint::LinkCtid => format!("/{api_version}/ctid/link"),
            CtraderEndpoint::UpdateTrader(login) => {
                format!("/{api_version}/webserv/traders/{login}")
            }
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
            CtraderEndpoint::UpdateTrader(_) => Method::PATCH
        }
    }
}
