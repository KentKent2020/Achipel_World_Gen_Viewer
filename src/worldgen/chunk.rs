pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 384;

pub struct Chunk {
    data: Vec<u32>,
    chunk_x: i32,
    chunk_z: i32,
}

impl Chunk {
    // Constructor to create a new Chunk
    pub fn new() -> Self {
        // Initialize chunk data with default values
        let data = vec![0; CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE];
        Chunk {
            data,
            chunk_x: 0,
            chunk_z: 0,
        }
    }

    // Method to set block at given coordinates
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: u32) {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            let index = self.get_index(x, y, z);
            self.data[index] = block_id;
        } else {
            panic!("Coordinates out of range");
        }
    }

    // Method to get block at given coordinates
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u32 {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            let index = self.get_index(x, y, z);
            self.data[index]
        } else {
            panic!("Coordinates out of range");
        }
    }

    // Method to set chunk coordinates
    pub fn set_coordinates(&mut self, chunk_x: i32, chunk_z: i32) {
        self.chunk_x = chunk_x;
        self.chunk_z = chunk_z;
    }

    // Method to get chunk coordinates
    pub fn get_coordinates(&self) -> (i32, i32) {
        (self.chunk_x, self.chunk_z)
    }

    // Method to get the index in the flat data vector from coordinates
    fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            x + (z * CHUNK_SIZE) + (y * CHUNK_SIZE * CHUNK_SIZE)
        } else {
            panic!("Coordinates out of range");
        }
    }

    pub fn get_blockcount(&self) -> u32 {
        let mut count = 0;
    
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    if self.get_block(x, y, z) != 0 {
                        count += 1;
                    }
                }
            }
        }
    
        count
    }
    
    pub fn get_bounds(&self) -> (u32, u32, u32, u32, u32, u32) {
        let tmpx = self.chunk_x as u32;
        let tmpz = self.chunk_z as u32;
    
        let min_x = tmpx * CHUNK_SIZE as u32;
        let min_z = tmpz * CHUNK_SIZE as u32;
        let min_y = 0;
        let max_x = min_x + CHUNK_SIZE as u32 - 1;
        let max_y = CHUNK_HEIGHT as u32;
        let max_z = min_z + CHUNK_SIZE as u32 - 1;
    
        (max_x, max_y, max_z, min_x, min_y, min_z)
    }

    pub fn get_realposition(&self, x: usize, y: usize, z: usize) -> (usize, usize, usize) {
        let pos_x = self.chunk_x;
        let pos_z = self.chunk_z;
    
        let chunk_corner_x = pos_x * 16;
        let chunk_corner_z = pos_z * 16;
    
        (
            (chunk_corner_x + x as i32) as usize,
            y,
            (chunk_corner_z + z as i32) as usize
        )
    }
    
}

