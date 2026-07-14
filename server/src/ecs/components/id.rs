use serde::Serialize;
use specs::{Component, FlaggedStorage};

#[derive(Default, Serialize, Clone, PartialEq)]
pub(crate) struct Player {
    pub(crate) id: u32,
    pub(crate) address: String,
}

impl Component for Player {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Default, Serialize, Clone)]
pub(crate) struct NetworkId(pub(crate) u32);

impl Component for NetworkId {
    type Storage = FlaggedStorage<Self>;
}
