use grpc_connect_server::grpc_connect_server::GrpcConnect;
use grpc_connect_server::{
    ConnectReply, ConnectRequest, GenHexIdReply, GenHexIdRequest, GenReceiverReply,
    GenReceiverRequest,
};
use tonic::{Request, Response, Status};

use crate::services;

pub mod grpc_connect_server {
    tonic::include_proto!("grpc_connect");
}

#[derive(Debug, Default)]
pub struct GrpcConnectSer {}

#[tonic::async_trait]
impl GrpcConnect for GrpcConnectSer {
    async fn connecting(
        &self,
        request: Request<ConnectRequest>,
    ) -> Result<Response<ConnectReply>, Status> {
        let req = request.into_inner();
        let ConnectRequest { service, request } = req;

        tracing::info!("Got a service: {:?}", service);
        tracing::info!("Got a request: {:?}", request);
        let res = services::connect::connect(service.as_str(), request).await;
        let reply = ConnectReply {
            message: res.unwrap(),
        };
        Ok(Response::new(reply)) //发回格式化的问候语
    }

    async fn gen_receiver(
        &self,
        request: Request<GenReceiverRequest>,
    ) -> Result<Response<GenReceiverReply>, Status> {
        let req = request.into_inner();
        let GenReceiverRequest {
            sender,
            receiver,
            path_code,
        } = req;
        let res =
            services::connect::gen_receiver(sender.as_str(), receiver.as_str(), path_code.as_str())
                .await;
        let reply = GenReceiverReply {
            receiver: res.unwrap(),
        };
        Ok(Response::new(reply)) //发回格式化的问候语
    }

    async fn gen_hex_id(
        &self,
        request: Request<GenHexIdRequest>,
    ) -> Result<Response<GenHexIdReply>, Status> {
        let req = request.into_inner();
        let GenHexIdRequest {
            username,
            receiver,
            path_code,
        } = req;
        let res =
            services::connect::gen_hex_id(username.as_str(), receiver.as_str(), path_code.as_str())
                .await;
        let reply = GenHexIdReply {
            hex_id: res.unwrap(),
        };
        Ok(Response::new(reply)) //发回格式化的问候语
    }
}
