/// The WebServices API is authenticated under the same manager credentials that are used to log into the cBroker application.
#[derive(Clone)]
pub struct ManagerCreds {
    pub password: String,
    pub login: i64,
}