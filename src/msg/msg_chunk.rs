pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgInChunkData {
    pub chunkx: u32,
    pub chunkz: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgOutChunkData {
    pub chunks: Vec<Vec<u32>>,
    pub data: Vec<Data>,
    pub blockCount: u32,
    pub bounds: Bounds,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bounds {
    pub maxX: u32,
    pub maxY: u32,
    pub maxZ: u32,
    pub minX: u32,
    pub minY: u32,
    pub minZ: u32,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub position: Vec<u32>,
    pub block: Block,
    pub biome: Biome,
    pub blockId: u32,
    pub blockData: u32,
    pub lighting: u32,
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub name: String,
    pub opaque: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Biome {
    pub name: String,
    pub temp: f32,
    pub humidity: f32,
}