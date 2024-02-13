use serde::{Deserialize, Serialize};
use crate::msg::msg_echo::MsgInEchoData;
use crate::msg::msg_chunk::MsgInChunkData;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "datainput", rename_all = "snake_case")]
pub enum MsgIn {
    Echo(MsgInEchoData),
    Chunk(MsgInChunkData)
}
