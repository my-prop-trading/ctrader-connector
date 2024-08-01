use crate::manager::api_client::ManagerApiConfig;
use crate::manager::common_messages_external::ProtoMessage;
use crate::manager::cs_messages_external::{ProtoCsPayloadType, ProtoManagerAuthReq};
use crate::manager::models::ManagerApiMessage;
use crate::manager::serialization::{ManagerApiSerializer, ManagerApiSerializerState};
use crate::utils::generate_password_hash;
use my_tcp_sockets::tcp_connection::TcpSocketConnection;
use my_tcp_sockets::SocketEventCallback;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[async_trait::async_trait]
pub trait ManagerApiCallbackHandler {
    async fn on_connected(&self);
    async fn on_disconnected(&self);
    async fn on_message(&self, event: ManagerApiMessage);
}

pub type ManagerApiConnection =
    TcpSocketConnection<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>;

pub struct ManagerApiCallback<T: ManagerApiCallbackHandler + Send + Sync + 'static> {
    handler: Arc<T>,
    config: Arc<ManagerApiConfig>,
    connection: RwLock<Option<Arc<ManagerApiConnection>>>,
    connect_timeout: Duration,
}

impl<T: ManagerApiCallbackHandler + Send + Sync + 'static> ManagerApiCallback<T> {
    pub fn new(handler: Arc<T>, config: Arc<ManagerApiConfig>, connect_timeout: Duration) -> Self {
        ManagerApiCallback {
            handler,
            config,
            connection: RwLock::new(None),
            connect_timeout,
        }
    }

    pub async fn is_connected(&self) -> bool {
        self.connection.read().await.is_some()
    }

    pub async fn wait_until_connected(&self) -> Result<(), String> {
        let instant = Instant::now();

        loop {
            if self.is_connected().await {
                // ensure full initialization
                tokio::time::sleep(Duration::from_millis(500)).await;
                return Ok(());
            }

            tokio::time::sleep(Duration::from_millis(250)).await;

            if instant.elapsed() > self.connect_timeout {
                return Err("Connect timeout".to_string());
            }
        }
    }

    pub async fn send<R: prost::Message>(
        &self,
        req: R,
        payload_type: ProtoCsPayloadType,
    ) -> Result<(), String> {
        while !self.is_connected().await {
            self.wait_until_connected().await?;
        }

        let connection_lock = self.connection.read().await;
        let connection = connection_lock.as_ref().expect("must exist");
        let message = ProtoMessage::new(req, payload_type);

        let Ok(message) = message else {
            return Err(format!(
                "Failed to create proto message: {:?}",
                message.unwrap_err()
            ));
        };

        connection.send(&message).await;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<T: ManagerApiCallbackHandler + Send + Sync + 'static>
    SocketEventCallback<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>
    for ManagerApiCallback<T>
{
    async fn connected(&self, connection: Arc<ManagerApiConnection>) {
        let req = ProtoManagerAuthReq {
            payload_type: Some(ProtoCsPayloadType::ProtoManagerAuthReq as i32),
            plant_id: self.config.plant_id.clone(),
            environment_name: self.config.env_name.clone(),
            login: self.config.creds.login,
            password_hash: generate_password_hash(&self.config.creds.password),
        };
        let mut bytes = vec![];
        prost::Message::encode(&req, &mut bytes).unwrap();
        let message = ProtoMessage {
            payload_type: ProtoCsPayloadType::ProtoManagerAuthReq as u32,
            payload: Some(bytes),
            client_msg_id: None,
        };
        connection.send(&message).await;
        let mut current_connection = self.connection.write().await;
        *current_connection = Some(connection.clone());
        drop(current_connection);
        self.handler.on_connected().await;
    }

    async fn disconnected(&self, _connection: Arc<ManagerApiConnection>) {
        let mut current_connection = self.connection.write().await;
        *current_connection = None;
        drop(current_connection);
        self.handler.on_disconnected().await;
    }

    async fn payload(&self, _connection: &Arc<ManagerApiConnection>, contract: ProtoMessage) {
        let message = ManagerApiMessage::try_from_proto(contract);

        let Ok(message) = message else {
            panic!("Failed to parse proto: {}", message.unwrap_err());
        };

        if let Some(message) = message {
            self.handler.on_message(message).await;
        }
    }
}
