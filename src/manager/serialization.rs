use crate::manager::common_messages_external::{ProtoMessage, ProtoPingReq};
use crate::manager::common_model_messages_external::ProtoPayloadType;
use crate::manager::cs_messages_external::ProtoCsPayloadType;
use async_trait::async_trait;
use my_tcp_sockets::{
    socket_reader::{ReadingTcpContractFail, SocketReader},
    TcpContract, TcpSerializerFactory, TcpSerializerState, TcpSocketSerializer, TcpWriteBuffer,
};
use prost::EncodeError;
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Default)]
pub struct ManagerApiSerializer {}

pub struct ManagerApiSerializerState {}

impl TcpSerializerState<ProtoMessage> for ManagerApiSerializerState {
    fn is_tcp_contract_related_to_metadata(&self, _contract: &ProtoMessage) -> bool {
        false
    }

    fn apply_tcp_contract(&mut self, _contract: &ProtoMessage) {}
}

impl ProtoMessage {
    pub fn new<P: prost::Message>(
        payload: P,
        payload_type: ProtoCsPayloadType,
    ) -> Result<ProtoMessage, EncodeError> {
        let mut payload_bytes = vec![];
        prost::Message::encode(&payload, &mut payload_bytes)?;

        Ok(ProtoMessage {
            payload_type: payload_type as u32,
            payload: Some(payload_bytes),
            client_msg_id: None,
        })
    }
}

#[async_trait]
impl TcpSocketSerializer<ProtoMessage, ManagerApiSerializerState> for ManagerApiSerializer {
    fn serialize(
        &self,
        out: &mut impl TcpWriteBuffer,
        contract: &ProtoMessage,
        _state: &ManagerApiSerializerState,
    ) {
        // Sending messages to the proxy follows the same basic principles as receiving, in that
        // the payload should be wrapped within the ProtoMessage structure,
        // and a 4-byte array indicating the length prepended to the generated byte array.
        let mut data_bytes = Vec::new();
        prost::Message::encode(contract, &mut data_bytes).unwrap();
        let len = data_bytes.len() as i32;
        let len_bytes = len.to_be_bytes();
        let mut bytes = Vec::with_capacity(len_bytes.len() + data_bytes.len());
        bytes.extend(len_bytes);
        bytes.extend(data_bytes);
        out.write_slice(&bytes[..]);
    }

    fn get_ping(&self) -> ProtoMessage {
        let payload_type = ProtoPayloadType::PingReq;
        let timestamp_micros = DateTimeAsMicroseconds::now().unix_microseconds as u64;
        let req = ProtoPingReq {
            payload_type: Some(payload_type as i32),
            timestamp: timestamp_micros / 1000,
        };
        let mut bytes = vec![];
        prost::Message::encode(&req, &mut bytes).unwrap();

        ProtoMessage {
            payload_type: payload_type as u32,
            payload: Some(bytes),
            client_msg_id: None,
        }
    }

    async fn deserialize<TSocketReader: Send + Sync + 'static + SocketReader>(
        &mut self,
        socket_reader: &mut TSocketReader,
        _state: &ManagerApiSerializerState,
    ) -> Result<ProtoMessage, ReadingTcpContractFail> {
        // When reading messages from the stream, the first 4 bytes indicate the length of the actual data.
        // The message which follows is always wrapped within the ProtoMessage structure.
        let mut len_buff = [0; 4];
        socket_reader.read_buf(&mut len_buff).await?;
        let len = i32::from_be_bytes(len_buff) as usize;
        let mut data_buf = Vec::with_capacity(len);
        // safety: we created vec with specified capacity
        unsafe { data_buf.set_len(len) }
        socket_reader.read_buf(&mut data_buf[..]).await?;
        let message: ProtoMessage = prost::Message::decode(&data_buf[..]).unwrap();

        Ok(message)
    }
}

impl TcpContract for ProtoMessage {
    fn is_pong(&self) -> bool {
        self.payload_type == ProtoPayloadType::PingRes as u32
    }
}

#[derive(Default)]
pub struct ManagerApiSerializerFactory {}

#[async_trait]
impl TcpSerializerFactory<ProtoMessage, ManagerApiSerializer, ManagerApiSerializerState>
    for ManagerApiSerializerFactory
{
    async fn create_serializer(&self) -> ManagerApiSerializer {
        ManagerApiSerializer::default()
    }

    async fn create_serializer_state(&self) -> ManagerApiSerializerState {
        ManagerApiSerializerState {}
    }
}
