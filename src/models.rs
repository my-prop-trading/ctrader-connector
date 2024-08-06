/// The APIs are authenticated under the same manager credentials that are used to log into the cBroker application.
#[async_trait::async_trait]
pub trait ManagerCreds {
    async fn get_password(&self) -> String;
    async fn get_login(&self) -> i64;
}
