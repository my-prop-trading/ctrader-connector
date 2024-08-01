use crate::manager::api_client::ManagerApiConfig;
use crate::manager::common_messages_external::ProtoMessage;
use crate::manager::cs_messages_external::{ProtoCsPayloadType, ProtoManagerAuthReq};
use crate::manager::models::ManagerApiMessage;
use crate::manager::serialization::{ManagerApiSerializer, ManagerApiSerializerState};
use crate::utils::generate_password_hash;
use my_tcp_sockets::tcp_connection::TcpSocketConnection;
use my_tcp_sockets::SocketEventCallback;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait::async_trait]
pub trait ManagerApiCallbackHandler {
    async fn on_connected(&self);
    async fn on_disconnected(&self);
    async fn on_event(&self, event: ManagerApiMessage);
}

pub struct ManagerApiCallback<T: ManagerApiCallbackHandler + Send + Sync + 'static> {
    handler: Arc<T>,
    config: Arc<ManagerApiConfig>,
    connection: RwLock<
        Option<
            Arc<TcpSocketConnection<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>>,
        >,
    >,
}

impl<T: ManagerApiCallbackHandler + Send + Sync + 'static> ManagerApiCallback<T> {
    pub fn new(handler: Arc<T>, config: Arc<ManagerApiConfig>) -> Self {
        ManagerApiCallback {
            handler,
            config,
            connection: RwLock::new(None),
        }
    }
}

#[async_trait::async_trait]
impl<T: ManagerApiCallbackHandler + Send + Sync + 'static>
    SocketEventCallback<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>
    for ManagerApiCallback<T>
{
    async fn connected(
        &self,
        connection: Arc<
            TcpSocketConnection<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>,
        >,
    ) {
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

    async fn disconnected(
        &self,
        _connection: Arc<
            TcpSocketConnection<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>,
        >,
    ) {
        let mut current_connection = self.connection.write().await;
        *current_connection = None;
        drop(current_connection);
        self.handler.on_disconnected().await;
    }

    async fn payload(
        &self,
        _connection: &Arc<
            TcpSocketConnection<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>,
        >,
        contract: ProtoMessage,
    ) {
        self.handler.on_event(contract.into()).await;
    }
}
