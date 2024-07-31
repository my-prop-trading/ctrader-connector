use crate::manager::serialization::{
    ManagerApiMessage, ManagerApiSerializer, ManagerApiSerializerState,
};
use my_tcp_sockets::tcp_connection::TcpSocketConnection;
use my_tcp_sockets::SocketEventCallback;
use std::sync::Arc;

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
    SocketEventCallback<ManagerApiMessage, ManagerApiSerializer, ManagerApiSerializerState>
    for ManagerApiCallback<T>
{
    async fn connected(
        &self,
        _connection: Arc<
            TcpSocketConnection<ManagerApiMessage, ManagerApiSerializer, ManagerApiSerializerState>,
        >,
    ) {
        self.handler.on_connected().await;
    }

    async fn disconnected(
        &self,
        _connection: Arc<
            TcpSocketConnection<ManagerApiMessage, ManagerApiSerializer, ManagerApiSerializerState>,
        >,
    ) {
        self.handler.on_disconnected().await;
    }

    async fn payload(
        &self,
        _connection: &Arc<
            TcpSocketConnection<ManagerApiMessage, ManagerApiSerializer, ManagerApiSerializerState>,
        >,
        contract: ManagerApiMessage,
    ) {
        self.handler.on_event(contract).await;
    }
}
