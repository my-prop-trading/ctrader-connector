use crate::manager::callback::{ManagerApiCallback, ManagerApiCallbackHandler};
use crate::manager::cs_messages_external::{
    ProtoCsPayloadType, ProtoManagerClosePositionReq, ProtoTraderListReq,
};
use crate::manager::serialization::ManagerApiSerializerFactory;
use crate::models::ManagerCreds;
use my_tcp_sockets::{TcpClient, TcpClientSocketSettings, TlsSettings};
use rust_extensions::Logger;
use std::sync::Arc;
use std::time::Duration;

pub struct ManagerApiClient<T: ManagerApiCallbackHandler + Send + Sync + 'static> {
    tcp_client: tokio::sync::Mutex<Option<TcpClient>>,
    logger: Arc<dyn Logger + Send + Sync + 'static>,
    inner_client: Arc<ManagerApiCallback<T>>,
    config_wrapper: Arc<ManagerApiConfigWrapper>,
}

impl<T: ManagerApiCallbackHandler + Send + Sync + 'static> ManagerApiClient<T> {
    pub fn new(
        handler: Arc<T>,
        config: Arc<dyn ManagerApiConfig + Send + Sync>,
        creds: Arc<dyn ManagerCreds + Send + Sync>,
        logger: Arc<dyn Logger + Send + Sync + 'static>,
    ) -> Self {
        let config_wrapper = Arc::new(ManagerApiConfigWrapper {
            config: Arc::clone(&config),
            creds,
        });
        let callback = ManagerApiCallback::new(
            handler,
            Arc::clone(&config_wrapper),
            Duration::from_secs(30),
            logger.clone(),
        );

        Self {
            inner_client: Arc::new(callback),
            tcp_client: Default::default(),
            logger,
            config_wrapper,
        }
    }

    pub async fn connect(&self) -> Result<(), String> {
        self.logger.write_info(
            "ManagerApiClient.connect".into(),
            "Starting tcp connection..".into(),
            None,
        );
        let domain_name = self.config_wrapper.get_domain().await;
        let tcp_client = TcpClient::new(domain_name, self.config_wrapper.clone())
            .set_disconnect_timeout(Duration::from_secs(40))
            .set_reconnect_timeout(Duration::from_secs(20))
            .set_seconds_to_ping(10);

        tcp_client
            .start(
                Arc::new(ManagerApiSerializerFactory::default()),
                Arc::clone(&self.inner_client),
                Arc::clone(&self.logger),
            )
            .await;
        self.tcp_client.lock().await.replace(tcp_client);

        self.inner_client.wait_until_connected().await?;

        Ok(())
    }

    pub async fn is_connected(&self) -> bool {
        self.inner_client.is_connected().await
    }

    pub async fn disconnect(&self) {
        let tcp_client = &*self.tcp_client.lock().await;

        if let Some(tcp_client) = tcp_client {
            tcp_client.stop().await;
        }
    }

    pub async fn req_close_position(
        &self,
        req: ProtoManagerClosePositionReq,
    ) -> Result<(), String> {
        let mut req = req;

        if req.channel.is_none() {
            req.channel = Some("ManagerAPI".to_string());
        }

        self.inner_client
            .send(req, ProtoCsPayloadType::ProtoManagerClosePositionReq)
            .await
    }

    pub async fn req_trader_list(&self, req: ProtoTraderListReq) -> Result<(), String> {
        self.inner_client
            .send(req, ProtoCsPayloadType::ProtoTraderListReq)
            .await
    }
}

pub struct ManagerApiConfigWrapper {
    pub config: Arc<dyn ManagerApiConfig + Send + Sync + 'static>,
    pub creds: Arc<dyn ManagerCreds + Send + Sync + 'static>,
}

#[async_trait::async_trait]
pub trait ManagerApiConfig {
    async fn get_url(&self) -> String;
    async fn get_plant_id(&self) -> String;
    async fn get_env_name(&self) -> String;
}

impl ManagerApiConfigWrapper {
    pub async fn get_domain(&self) -> String {
        let url = self.config.get_url().await;
        let mut splits = url.split(':');

        splits.next().unwrap().to_string()
    }
}

#[async_trait::async_trait]
impl TcpClientSocketSettings for ManagerApiConfigWrapper {
    async fn get_host_port(&self) -> Option<String> {
        let url = self.config.get_url().await;
        Some(url)
    }

    async fn get_tls_settings(&self) -> Option<TlsSettings> {
        Some(TlsSettings {
            server_name: self.get_domain().await,
        })
    }
}
