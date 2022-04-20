#![allow(unused)]
#![feature(box_into_inner)]
use tonic::transport::Server;

mod rpc;
mod services;
mod util;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        tracing_subscriber::fmt().init();
    } else {
        tracing_subscriber::fmt::init();
    }
    tracing::info!("start...");
    std::fs::create_dir_all("./storage/grpc_connect").unwrap();
    let addr = "0.0.0.0:50051".parse().unwrap();
    let grpc_connect = rpc::server::GrpcConnectSer::default();
    // let group_grpc = rpc::server::GroupSer::default();

    Server::builder()
        .http2_keepalive_interval(Some(std::time::Duration::from_secs(5)))
        .tcp_keepalive(Some(std::time::Duration::from_secs(5)))
        .add_service(
            rpc::server::grpc_connect_server::grpc_connect_server::GrpcConnectServer::new(
                grpc_connect,
            ),
        )
        // .add_service(rpc::server::group_grpc_server::group_server::GroupServer::new(group_grpc))
        .serve(addr)
        .await
        .unwrap();
}
