use specs::Component;

pub(crate) enum SpriteType {
    Human,
}

impl ToString for SpriteType {
    fn to_string(&self) -> String {
        match self {
            SpriteType::Human => "human".to_string(),
        }
    }
}

pub(crate) struct Sprite(pub(crate) SpriteType);

impl Component for Sprite {
    type Storage = specs::FlaggedStorage<Self>;
}
