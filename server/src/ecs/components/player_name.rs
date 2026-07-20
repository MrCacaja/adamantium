use specs::{Component, FlaggedStorage};

#[derive(Default, Clone)]
pub(crate) struct PlayerName(pub(crate) String);

impl Component for PlayerName {
    type Storage = FlaggedStorage<Self>;
}
