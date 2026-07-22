use std::collections::HashMap;

use crate::log::{Log, LogLevel};
use crate::world::{
    chunk::{Chunk, ChunkCoord, CHUNK_SIZE},
    generator::WorldGenerator,
    persistence::WorldPersistence,
    tile::TileType,
};

pub struct WorldManager {
    pub generator: WorldGenerator,
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub persistence: WorldPersistence,
    pub load_radius: i32,
    pub dirty_chunks: Vec<ChunkCoord>,
}

impl WorldManager {
    pub fn new(seed: u64, persistence: WorldPersistence, load_radius: i32) -> Self {
        let generator = WorldGenerator::new(seed);
        Log::new(
            LogLevel::Info,
            format!("WorldManager initialized with seed: {}", seed),
        );

        Self {
            generator,
            chunks: HashMap::new(),
            persistence,
            load_radius,
            dirty_chunks: Vec::new(),
        }
    }

    pub fn get_or_generate_chunk(&mut self, coord: ChunkCoord) -> &Chunk {
        if !self.chunks.contains_key(&coord) {
            let chunk = self.generator.generate_chunk(coord);
            self.chunks.insert(coord, chunk);
        }

        self.chunks.get(&coord).unwrap()
    }

    pub fn get_chunk(&self, coord: ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(&coord)
    }

    pub fn world_to_tile(&self, world_x: f32, world_y: f32) -> Option<(ChunkCoord, usize, usize)> {
        let coord = ChunkCoord::from_world_pos(world_x, world_y);
        if let Some(chunk) = self.chunks.get(&coord) {
            let (tx, ty) = chunk.world_to_tile(world_x, world_y);
            Some((coord, tx, ty))
        } else {
            None
        }
    }

    pub fn load_chunks_around(&mut self, x: f32, y: f32) -> Vec<ChunkCoord> {
        let center = ChunkCoord::from_world_pos(x, y);
        let mut loaded = Vec::new();

        for dy in -self.load_radius..=self.load_radius {
            for dx in -self.load_radius..=self.load_radius {
                let coord = ChunkCoord::new(center.x + dx, center.y + dy);
                if !self.chunks.contains_key(&coord) {
                    self.get_or_generate_chunk(coord);
                }
                loaded.push(coord);
            }
        }

        loaded
    }

    pub fn unload_distant_chunks(&mut self, active_positions: &[(f32, f32)]) {
        if active_positions.is_empty() {
            return;
        }

        let min_distance = (self.load_radius + 2) as f32 * CHUNK_SIZE as f32;

        let chunks_to_remove: Vec<ChunkCoord> = self
            .chunks
            .keys()
            .filter(|coord| {
                let ((min_x, min_y), (max_x, max_y)) = coord.world_pos_range();
                let center_x = (min_x + max_x) / 2.0;
                let center_y = (min_y + max_y) / 2.0;

                active_positions.iter().all(|(px, py)| {
                    let dx = center_x - px;
                    let dy = center_y - py;
                    (dx * dx + dy * dy).sqrt() > min_distance
                })
            })
            .cloned()
            .collect();

        for coord in chunks_to_remove {
            self.chunks.remove(&coord);
        }
    }

    pub fn serialize_chunk(&self, coord: ChunkCoord) -> Option<ChunkPayload> {
        let chunk = self.chunks.get(&coord)?;

        let tiles = chunk.tiles_as_ids();

        Some(ChunkPayload {
            chunk_x: coord.x,
            chunk_y: coord.y,
            tiles,
        })
    }

    pub fn get_tile_at(&self, world_x: f32, world_y: f32) -> Option<TileType> {
        let (coord, tx, ty) = self.world_to_tile(world_x, world_y)?;
        let chunk = self.chunks.get(&coord)?;
        Some(chunk.get_tile(tx, ty))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ChunkPayload {
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub tiles: Vec<Vec<u8>>,
}
