use serde::{Deserialize, Serialize};

use super::tile::TileType;

pub const CHUNK_SIZE: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_world_pos(x: f32, y: f32) -> Self {
        Self {
            x: (x / CHUNK_SIZE as f32).floor() as i32,
            y: (y / CHUNK_SIZE as f32).floor() as i32,
        }
    }

    pub fn world_pos_range(&self) -> ((f32, f32), (f32, f32)) {
        let min_x = self.x as f32 * CHUNK_SIZE as f32;
        let min_y = self.y as f32 * CHUNK_SIZE as f32;
        let max_x = min_x + CHUNK_SIZE as f32;
        let max_y = min_y + CHUNK_SIZE as f32;
        ((min_x, min_y), (max_x, max_y))
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub tiles: [[TileType; CHUNK_SIZE]; CHUNK_SIZE],
    pub dirty: bool,
}

impl Chunk {
    pub fn new(coord: ChunkCoord, tiles: [[TileType; CHUNK_SIZE]; CHUNK_SIZE]) -> Self {
        Self {
            coord,
            tiles,
            dirty: false,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileType {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            self.tiles[y][x]
        } else {
            TileType::Grass
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile_type: TileType) {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            self.tiles[y][x] = tile_type;
            self.dirty = true;
        }
    }

    pub fn world_to_tile(&self, world_x: f32, world_y: f32) -> (usize, usize) {
        let ((min_x, min_y), _) = self.coord.world_pos_range();
        let tx = (world_x - min_x).floor() as usize;
        let ty = (world_y - min_y).floor() as usize;
        (tx.min(CHUNK_SIZE - 1), ty.min(CHUNK_SIZE - 1))
    }

    pub fn tiles_as_ids(&self) -> Vec<Vec<u8>> {
        self.tiles
            .iter()
            .map(|row| row.iter().map(|t| t.to_id()).collect())
            .collect()
    }
}
