use super::model::{LatestMessage, NewMessage, Req, UnreadStatus};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

pub(crate) async fn new_message(request: String) -> Result<String, String> {
    let req: NewMessage = serde_json::from_str(&request).unwrap();
    let request = tonic::Request::new(crate::rpc::client::message_grpc::NewRequest {
        path_code: req.path_code,
        message_type: req.message_type,
        message_id: req.message_id,
        content: req.content,
        timestamp: req.timestamp,
        trace_id: req.trace_id,
        sender: req.sender,
        receiver: req.receiver,
    });

    Ok(crate::rpc::message_client()
        .await
        .new_message(request)
        .await
        .map(|res| {
            tracing::info!("res: {:?}", res);
            let crate::rpc::client::message_grpc::NewReply { ok } = res.into_inner();
            ok.to_string()
        })
        .map_err(|e| e.to_string())?)
}

pub(crate) async fn latest_messages(request: String) -> Result<String, String> {
    let req: LatestMessage = serde_json::from_str(&request).unwrap();
    let request = tonic::Request::new(crate::rpc::client::message_grpc::LatestMessagesRequest {
        path_code: req.path_code,
        trace_id: req.trace_id,
        receiver: req.receiver,
        len: req.len,
    });

    Ok(crate::rpc::message_client()
        .await
        .latest_messages(request)
        .await
        .map(|res| {
            // tracing::info!("res: {:?}", res);
            let crate::rpc::client::message_grpc::LatestMessagesReply { messages } =
                res.into_inner();
            tracing::info!("messages: {:?}", messages);
            "ok".to_string()
        })
        .map_err(|e| e.to_string())?)
}

pub(crate) async fn unread_status(request: String) -> Result<String, String> {
    let req: UnreadStatus = serde_json::from_str(&request).unwrap();
    let list = req
        .list
        .iter()
        .map(|r| {
            let re: Req = serde_json::from_str(r).unwrap();
            tracing::info!("[unread_status]: space_code {:?}", &re.space_code);
            tracing::info!("[unread_status]: username {:?}", &req.username);
            tracing::info!("[unread_status]: receiver {:?}", &re.receiver);
            let receiver = gen_receiver(
                &re.space_code.clone(),
                &req.username.clone(),
                re.receiver.as_str(),
            );
            tracing::info!("[unread_status]: gen receiver {:?}", receiver);
            crate::rpc::client::message_grpc::unread_status_request::SpaceCodeAndReceiver {
                space_code: re.space_code,
                receiver: re.receiver,
            }
        })
        .collect();

    let request = tonic::Request::new(crate::rpc::client::message_grpc::UnreadStatusRequest {
        username: req.username,
        list,
        trace_id: req.trace_id,
    });

    Ok(crate::rpc::message_client()
        .await
        .unread_status(request)
        .await
        .map(|res| {
            // tracing::info!("res: {:?}", res);
            let crate::rpc::client::message_grpc::UnreadStatusReply { status } = res.into_inner();
            tracing::info!("status: {:?}", status);
            "ok".to_string()
        })
        .map_err(|e| e.to_string())?)
}

fn gen_receiver(space_code: &str, sender: &str, receiver: &str) -> String {
    if receiver == "" {
        String::new()
    } else {
        let mut hasher = DefaultHasher::new();
        if space_code.eq(sender) {
            format!("{}, {}", space_code, receiver).hash(&mut hasher);
        } else {
            format!("{}, {}", space_code, sender).hash(&mut hasher);
        }
        let receiver = hasher.finish();
        format!("{:x}", receiver)
    }
}
