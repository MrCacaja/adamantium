use serde::Serialize;
use specs::{Component, FlaggedStorage};

#[derive(Default, Serialize, Clone, PartialEq)]
pub(crate) enum Direction {
    #[default]
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
            Direction::Up => "up",
        }
    }
}

impl Component for Direction {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Default, Serialize, Clone, PartialEq)]
pub(crate) struct AnimState(pub String);

impl Component for AnimState {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Default)]
pub(crate) struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Component for Position {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Default, Serialize, Clone)]
pub(crate) struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = FlaggedStorage<Self>;
}
