use crate::worldgen::chunk::Chunk;

const CHUNK_HEIGHT: usize = 384;

pub fn gen_chunk(pos_x: i32, pos_z: i32, seed: u32) -> Chunk {
    let mut chunk = Chunk::new();
    chunk.set_coordinates(pos_x, pos_z);

    for x_k in 0..16 {
        for z_k in 0..16 {
            for y_k in 0..CHUNK_HEIGHT {
                if air_or_stone(x_k, y_k, z_k) {
                    chunk.set_block(x_k, y_k, z_k, 0);
                } else {
                    chunk.set_block(x_k, y_k, z_k, 1);
                }
            }
        }
    }

    chunk
}

fn air_or_stone(pos_x: usize, pos_y: usize, pos_z: usize) -> bool {
    pos_y > 100 // Example condition for simplicity
}
