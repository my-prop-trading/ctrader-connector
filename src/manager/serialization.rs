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

impl TcpSerializerState<ManagerApiMessage> for ManagerApiSerializerState {
    fn is_tcp_contract_related_to_metadata(&self, contract: &ManagerApiMessage) -> bool {
        println!("is_tcp_contract_related_to_metadata");
        // todo
        false
    }

    fn apply_tcp_contract(&mut self, contract: &ManagerApiMessage) {
        println!("apply_tcp_contract");
        // todo
    }
}

#[async_trait]
impl TcpSocketSerializer<ManagerApiMessage, ManagerApiSerializerState> for ManagerApiSerializer {
    fn serialize(
        &self,
        out: &mut impl TcpWriteBuffer,
        contract: &ManagerApiMessage,
        state: &ManagerApiSerializerState,
    ) {
        println!("serialize");
        // todo
    }

    fn get_ping(&self) -> ManagerApiMessage {
        println!("get_ping");

        ManagerApiMessage::HeartbeatEvent
    }

    async fn deserialize<TSocketReader: Send + Sync + 'static + SocketReader>(
        &mut self,
        socket_reader: &mut TSocketReader,
        state: &ManagerApiSerializerState,
    ) -> Result<ManagerApiMessage, ReadingTcpContractFail> {
        println!("deserialize");
        // todo
        Ok(ManagerApiMessage::Empty)
    }
}

impl ManagerApiMessage {
    pub fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = vec![];
        // todo
        //prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        let s = crate::manager::common_messages_external::ProtoMessage {
            payload_type: 0,
            payload: None,
            client_msg_id: None,
        };
        // todo
        //prost::Message::decode(bytes)
        Ok(ManagerApiMessage::Empty)
    }
}

#[derive(Debug, Clone)]
pub enum ManagerApiMessage {
    Empty,
    HelloEvent,
    HeartbeatEvent,
}

impl TcpContract for ManagerApiMessage {
    fn is_pong(&self) -> bool {
        println!("is_pong");

        matches!(self, ManagerApiMessage::HeartbeatEvent)
    }
}

pub struct ManagerApiSerializerFactory {}

#[async_trait]
impl TcpSerializerFactory<ManagerApiMessage, ManagerApiSerializer, ManagerApiSerializerState>
    for ManagerApiSerializerFactory
{
    async fn create_serializer(&self) -> ManagerApiSerializer {
        ManagerApiSerializer::new()
    }

    async fn create_serializer_state(&self) -> ManagerApiSerializerState {
        ManagerApiSerializerState {}
    }
}
