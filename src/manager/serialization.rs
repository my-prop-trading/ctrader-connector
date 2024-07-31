use async_trait::async_trait;
use my_tcp_sockets::{
    socket_reader::{ReadBuffer, ReadingTcpContractFail, SocketReader},
    TcpContract, TcpSerializerFactory, TcpSerializerState, TcpSocketSerializer, TcpWriteBuffer,
};

pub struct ManagerApiSerializer {
    read_buffer: ReadBuffer,
}

impl ManagerApiSerializer {
    pub fn new() -> Self {
        Self {
            read_buffer: ReadBuffer::new(1024 * 24),
        }
    }
}

pub struct ManagerApiSerializerState {}

impl TcpSerializerState<ManagerApiEvent> for ManagerApiSerializerState {
    fn is_tcp_contract_related_to_metadata(&self, contract: &ManagerApiEvent) -> bool {
        todo!()
    }

    fn apply_tcp_contract(&mut self, contract: &ManagerApiEvent) {
        todo!()
    }
}

#[async_trait]
impl TcpSocketSerializer<ManagerApiEvent, ManagerApiSerializerState> for ManagerApiSerializer {
    fn serialize(
        &self,
        out: &mut impl TcpWriteBuffer,
        contract: &ManagerApiEvent,
        state: &ManagerApiSerializerState,
    ) {
        todo!()
    }

    fn get_ping(&self) -> ManagerApiEvent {
        todo!()
    }

    async fn deserialize<TSocketReader: Send + Sync + 'static + SocketReader>(
        &mut self,
        socket_reader: &mut TSocketReader,
        state: &ManagerApiSerializerState,
    ) -> Result<ManagerApiEvent, ReadingTcpContractFail> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum ManagerApiEvent {
    HelloEvent,
    HeartbeatEvent,
}

impl TcpContract for ManagerApiEvent {
    fn is_pong(&self) -> bool {
        todo!()
    }
}

pub struct ManagerApiSerializerFactory {}

#[async_trait]
impl TcpSerializerFactory<ManagerApiEvent, ManagerApiSerializer, ManagerApiSerializerState>
    for ManagerApiSerializerFactory
{
    async fn create_serializer(&self) -> ManagerApiSerializer {
        ManagerApiSerializer::new()
    }

    async fn create_serializer_state(&self) -> ManagerApiSerializerState {
        ManagerApiSerializerState {}
    }
}
