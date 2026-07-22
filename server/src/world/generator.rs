use noise::{NoiseFn, OpenSimplex};

use super::chunk::{Chunk, ChunkCoord, CHUNK_SIZE};
use super::tile::TileType;

pub struct WorldGenerator {
    height_noise: OpenSimplex,
    moisture_noise: OpenSimplex,
    detail_noise: OpenSimplex,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        let height_noise = OpenSimplex::new(seed as u32);
        let moisture_noise = OpenSimplex::new((seed + 1) as u32);
        let detail_noise = OpenSimplex::new((seed + 2) as u32);

        Self {
            height_noise,
            moisture_noise,
            detail_noise,
        }
    }

    pub fn generate_chunk(&self, coord: ChunkCoord) -> Chunk {
        let mut tiles = [[TileType::Grass; CHUNK_SIZE]; CHUNK_SIZE];

        let scale = 0.02;

        for ty in 0..CHUNK_SIZE {
            for tx in 0..CHUNK_SIZE {
                let world_x = coord.x as f64 * CHUNK_SIZE as f64 + tx as f64;
                let world_y = coord.y as f64 * CHUNK_SIZE as f64 + ty as f64;

                let nx = world_x * scale;
                let ny = world_y * scale;

                let height = self.sample_height(nx, ny);
                let moisture = self.sample_moisture(nx, ny);
                let detail = self.sample_detail(nx, ny);

                tiles[ty][tx] = self.get_tile_type(height, moisture, detail);
            }
        }

        Chunk::new(coord, tiles)
    }

    fn sample_height(&self, nx: f64, ny: f64) -> f64 {
        let base = self.height_noise.get([nx, ny]);
        let octave1 = self.height_noise.get([nx * 2.0, ny * 2.0]) * 0.5;
        let octave2 = self.height_noise.get([nx * 4.0, ny * 4.0]) * 0.25;

        let combined = (base + octave1 + octave2) / 1.75;
        (combined + 1.0) / 2.0
    }

    fn sample_moisture(&self, nx: f64, ny: f64) -> f64 {
        let base = self.moisture_noise.get([nx * 1.5, ny * 1.5]);
        let detail = self.moisture_noise.get([nx * 3.0, ny * 3.0]) * 0.3;

        let combined = (base + detail) / 1.3;
        (combined + 1.0) / 2.0
    }

    fn sample_detail(&self, nx: f64, ny: f64) -> f64 {
        let val = self.detail_noise.get([nx * 8.0, ny * 8.0]);
        (val + 1.0) / 2.0
    }

    fn get_tile_type(&self, height: f64, moisture: f64, detail: f64) -> TileType {
        if height < 0.2 {
            TileType::DeepWater
        } else if height < 0.3 {
            TileType::Water
        } else if height < 0.35 {
            TileType::Sand
        } else if height < 0.7 {
            if moisture > 0.35 {
                TileType::Grass
            } else {
                TileType::Dirt
            }
        } else if height < 0.85 {
            TileType::Stone
        } else {
            if detail > 0.4 {
                TileType::Snow
            } else {
                TileType::Stone
            }
        }
    }
}
