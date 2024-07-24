use http::Method;

pub enum WebservicesApiEndpoint {
    CreateManagerToken,
    CreateCtid,
    CreateTrader,
    LinkCtid,
    /// Changes of a trader entity (including allocating/removing credit).
    /// Requires {login}
    UpdateTrader(String),
    /// Changes the balance of a trader entity (including allocating/removing credit).
    /// Requires {login}
    UpdateTraderBalance(String),
}

impl From<&WebservicesApiEndpoint> for String {
    fn from(item: &WebservicesApiEndpoint) -> Self {
        let api_version = "v2";

        match item {
            WebservicesApiEndpoint::CreateManagerToken => {
                format!("/{}/webserv/managers/token", api_version)
            }
            WebservicesApiEndpoint::CreateCtid => format!("/{api_version}/ctid/create"),
            WebservicesApiEndpoint::CreateTrader => format!("/{api_version}/webserv/traders"),
            WebservicesApiEndpoint::LinkCtid => format!("/{api_version}/ctid/link"),
            WebservicesApiEndpoint::UpdateTrader(login) => {
                format!("/{api_version}/webserv/traders/{login}")
            }
            WebservicesApiEndpoint::UpdateTraderBalance(login) => {
                format!("/{api_version}/webserv/traders/{login}/changebalance")
            }
        }
    }
}

impl WebservicesApiEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            WebservicesApiEndpoint::CreateManagerToken => Method::POST,
            WebservicesApiEndpoint::CreateCtid => Method::POST,
            WebservicesApiEndpoint::CreateTrader => Method::POST,
            WebservicesApiEndpoint::LinkCtid => Method::POST,
            WebservicesApiEndpoint::UpdateTrader(_) => Method::PATCH,
            WebservicesApiEndpoint::UpdateTraderBalance(_) => Method::POST,
        }
    }
}
