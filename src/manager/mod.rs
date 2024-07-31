pub mod api_client;
pub mod models;
pub mod serialization;
pub mod callback;

pub mod common_messages_external {
    tonic::include_proto!("common_messages_external");
}

pub mod common_model_messages_external {
    tonic::include_proto!("common_model_messages_external");
}

pub mod cs_messages_external {
    tonic::include_proto!("cs_messages_external");
}

pub mod cs_model_messages_external {
    tonic::include_proto!("cs_model_messages_external");
}