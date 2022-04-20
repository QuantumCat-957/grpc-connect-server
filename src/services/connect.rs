use super::handler;
use crate::util::TimeRecord;
use serde_json::to_string;
use std::{
    collections::{hash_map::DefaultHasher, BTreeSet, HashMap},
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

pub(crate) async fn connect(service: &str, request: String) -> Result<String, String> {
    match service {
        "new_message" => handler::new_message(request).await,
        "latest_messages" => handler::latest_messages(request).await,
        "unread_status" => handler::unread_status(request).await,
        _ => Ok(String::new()),
    }
}

pub(crate) async fn gen_receiver(
    sender: &str,
    receiver: &str,
    path_code: &str,
) -> Result<String, String> {
    Ok(if receiver == "" {
        String::new()
    } else {
        let mut hasher = DefaultHasher::new();
        if path_code.eq(sender) {
            format!("{}, {}", path_code, receiver).hash(&mut hasher);
        } else {
            format!("{}, {}", path_code, sender).hash(&mut hasher);
        }
        let receiver = hasher.finish();
        format!("{:x}", receiver)
    })
}

pub(crate) async fn gen_hex_id(
    username: &str,
    receiver: &str,
    path_code: &str,
) -> Result<String, String> {
    let mut hasher = DefaultHasher::new();
    format!("{}{}{}", username, path_code, receiver).hash(&mut hasher);
    let receiver = hasher.finish();
    Ok(format!("{:x}", receiver))
}
