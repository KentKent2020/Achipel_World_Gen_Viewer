pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgOutChunkEndData {
    pub endmessage: String,
}
