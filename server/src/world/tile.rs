use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileType {
    Grass = 0,
    Dirt = 1,
    Sand = 2,
    Water = 3,
    Stone = 4,
    DeepWater = 5,
    Snow = 6,
}

impl TileType {
    pub fn to_id(self) -> u8 {
        self as u8
    }
}
