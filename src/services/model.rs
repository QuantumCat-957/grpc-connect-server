use prost::encoding::int64;
use serde_derive::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct NewMessage {
    // #[serde(rename = "spaceType")]
    pub(crate) path_code: String,
    // #[serde(rename = "avatarUrl")]
    pub(crate) message_type: String,
    pub(crate) message_id: String,
    pub(crate) content: String,
    pub(crate) timestamp: String,
    #[serde(rename = "traceID")]
    pub(crate) trace_id: i64,
    pub(crate) receiver: String,
    pub(crate) sender: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LatestMessage {
    pub(crate) path_code: String,
    // #[serde(rename = "avatarUrl")]
    pub(crate) receiver: String,
    pub(crate) len: u32,
    #[serde(rename = "traceID")]
    pub(crate) trace_id: i64,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct UnreadStatus {
    pub(crate) username: String,
    pub(crate) list: Vec<String>,
    #[serde(rename = "traceID")]
    pub(crate) trace_id: i64,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Req {
    pub(crate) space_code: String,
    pub(crate) receiver: String,
}
