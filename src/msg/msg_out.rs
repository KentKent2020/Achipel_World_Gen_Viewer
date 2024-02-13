use serde::{Deserialize, Serialize};
use crate::msg::msg_echo::MsgOutEchoData;
use crate::msg::msg_chunk::MsgOutChunkData;
use crate::msg::msg_chunkend::MsgOutChunkEndData;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "dataoutput", rename_all = "snake_case")]
pub enum MsgOut {
    Echo(MsgOutEchoData),
    Chunk(MsgOutChunkData),
    ChunkEnd(MsgOutChunkEndData)
}
