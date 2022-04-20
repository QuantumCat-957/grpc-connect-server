pub mod client;
pub mod server;

pub(crate) use client::message_grpc::message_client::MessageClient;

pub(crate) async fn message_client() -> MessageClient<tonic::transport::Channel> {
    MessageClient::connect("http://levitas.quakeai.tech:31045")
    // MessageClient::connect("http://127.0.0.1:20001")
        .await
        .unwrap()
}
