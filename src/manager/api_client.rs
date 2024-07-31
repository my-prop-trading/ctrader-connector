use crate::manager::callback::{ManagerApiCallback, ManagerApiCallbackHandler};
use crate::manager::serialization::ManagerApiSerializerFactory;
use crate::webservices::creds::ManagerCreds;
use my_tcp_sockets::{TcpClient, TcpClientSocketSettings, TlsSettings};
use rust_extensions::Logger;
use std::sync::Arc;

pub struct ManagerApiClient<T: ManagerApiCallbackHandler + Send + Sync + 'static> {
    tcp_client: TcpClient,
    logger: Arc<dyn Logger + Send + Sync + 'static>,
    handler: Arc<T>,
    config: Arc<ManagerApiConfig>,
}

impl<T: ManagerApiCallbackHandler + Send + Sync + 'static> ManagerApiClient<T> {
    pub fn new(
        handler: Arc<T>,
        config: Arc<ManagerApiConfig>,
        logger: Arc<dyn Logger + Send + Sync + 'static>,
    ) -> Self {
        Self {
            tcp_client: TcpClient::new(config.server_name.clone(), config.clone()),
            logger,
            handler,
            config,
        }
    }

    pub async fn connect(&self) -> Result<(), String> {
        self.tcp_client
            .start(
                Arc::new(ManagerApiSerializerFactory::default()),
                Arc::new(ManagerApiCallback::new(
                    Arc::clone(&self.handler),
                    Arc::clone(&self.config),
                )),
                Arc::clone(&self.logger),
            )
            .await;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ManagerApiConfig {
    pub server_name: String,
    pub host_port: String,
    pub creds: ManagerCreds,
    pub plant_id: String,
    pub env_name: String,
}

#[async_trait::async_trait]

impl TcpClientSocketSettings for ManagerApiConfig {
    async fn get_host_port(&self) -> Option<String> {
        Some(self.host_port.clone())
    }

    async fn get_tls_settings(&self) -> Option<TlsSettings> {
        Some(TlsSettings {
            server_name: self.server_name.clone(),
        })
    }
}