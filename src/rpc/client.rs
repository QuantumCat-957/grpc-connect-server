use message_grpc::message_client::MessageClient;
use message_grpc::NewRequest;
pub mod message_grpc{
    tonic::include_proto!("message_grpc");
}

// #[tokio::main]
// async fn main() -> Result<(),Box<dyn std::error::Error>>{
//     let mut client = MessageClient::connect("http://127.0.0.1:20001").await?;

//     let request = tonic::Request::new(NewRequest{
//         username: "Hello".to_owned(),
//   path_code: "Hello".to_owned(),
//   message_type: "Hello".to_owned(),
//   message_id: "8400c632-253e-4f30-be1d-3f899a1113df".to_owned(),
//   content: "Hello".to_owned(),
//   timestamp: "Hello".to_owned(),
//   trace_id: 20
//     });

//     let response = client.new_message(request).await?;

//     println!("Response={:?}",response);

//     Ok(())
// }