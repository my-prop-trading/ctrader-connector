use crate::manager::common_messages_external::ProtoMessage;
use crate::manager::serialization::{ManagerApiSerializer, ManagerApiSerializerState};
use my_tcp_sockets::tcp_connection::TcpSocketConnection;
use my_tcp_sockets::SocketEventCallback;
use std::sync::Arc;
use crate::manager::models::ManagerApiMessage;

#[async_trait::async_trait]
pub trait ManagerApiCallbackHandler {
    async fn on_connected(&self);
    async fn on_disconnected(&self);
    async fn on_event(&self, event: ManagerApiMessage);
}

pub struct ManagerApiCallback<T: ManagerApiCallbackHandler + Send + Sync + 'static> {
    handler: Arc<T>,
}

impl<T: ManagerApiCallbackHandler + Send + Sync + 'static> ManagerApiCallback<T> {
    pub fn new(handler: Arc<T>) -> Self {
        ManagerApiCallback { handler }
    }
}

#[async_trait::async_trait]
impl<T: ManagerApiCallbackHandler + Send + Sync + 'static>
SocketEventCallback<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>
for ManagerApiCallback<T>
{
    async fn connected(
        &self,
        _connection: Arc<
            TcpSocketConnection<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>,
        >,
    ) {
        self.handler.on_connected().await;
    }

    async fn disconnected(
        &self,
        _connection: Arc<
            TcpSocketConnection<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>,
        >,
    ) {
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

impl From<ProtoMessage> for ManagerApiMessage {
    fn from(value: ProtoMessage) -> Self {
        let payload_type = value.payload_type as i32;

        if let Some(event) = ManagerApiMessage::try_from_common(payload_type, &value.payload) {
            return event;
        }

        if let Some(event) = ManagerApiMessage::try_from_cs(payload_type, &value.payload) {
            return event;
        }
        
        println!("{:?}", value);        
        panic!("failed to parse");
    }
}