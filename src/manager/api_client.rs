pub struct ManagerApiClient {
    url: String,
}

impl ManagerApiClient {
    pub fn new(domain: impl Into<String>, port: usize) -> Self {
        Self {
            url: "".to_string(),
        }
    }

    pub async fn connect() -> Result<(), String> {
        Ok(())
    }
}
