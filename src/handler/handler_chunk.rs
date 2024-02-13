use crate::msg::msg_chunk::{Biome, Block, Data, MsgInChunkData, MsgOutChunkData, Bounds};
use crate::worldgen::maingen::gen_chunk;

// Déclaration de la variable globale mutable
pub static mut INDEXKENT: u32 = 0;

// Fonction pour gérer le message du chunk
pub fn handle_chunk(msg: &MsgInChunkData, pos_x: i32, pos_z: i32) -> MsgOutChunkData {
    // Générer les données du chunk
    let chunk = gen_chunk(pos_x, pos_z, 6363); // Changer la seed si nécessaire

    // Récupérer le nombre de blocs et les limites
    let block_count = chunk.get_blockcount();
    let bounds = chunk.get_bounds();

    // Préparer les données du chunk
    let mut chunksdata = Vec::with_capacity(block_count as usize);

    // Remplir chunksdata avec les données des blocs
    for y in 0..384 {
        for z in 0..16 {
            for x in 0..16 {
                let rpob = chunk.get_realposition(x, y, z); // Position réelle du bloc dans le monde

                // Utiliser l'index global
                let tmpdata = Data {
                    position: vec![rpob.0 as u32, rpob.1 as u32, rpob.2 as u32],
                    block: Block {
                        name: String::from("Stone"),
                        opaque: Some(true),
                    },
                    biome: Biome {
                        name: String::from("Hills"),
                        temp: 0.5,
                        humidity: 0.5,
                    },
                    blockId: 1, // chunk.get_block(x, y, z),
                    blockData: 0,
                    lighting: 0,
                    index: unsafe { INDEXKENT },
                };

                // Incrémenter l'index global
                unsafe {
                    INDEXKENT += 1;
                }

                chunksdata.push(tmpdata);
            }
        }
    }

    // Créer et retourner la structure MsgOutChunkData
    MsgOutChunkData {
        chunks: vec![vec![pos_x as u32, pos_z as u32]],
        data: chunksdata,
        blockCount: block_count as u32,
        bounds: Bounds {
            maxX: bounds.0,
            maxY: bounds.1,
            maxZ: bounds.2,
            minX: bounds.3,
            minY: bounds.4,
            minZ: bounds.5,
        },
    }
}
