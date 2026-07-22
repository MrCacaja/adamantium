use serde::Serialize;

use crate::ecs::components::{
    id::NetworkId,
    id::Player,
    sprite::Sprite,
    transform::{AnimState, Direction, Position, Velocity},
};

#[derive(Default, Serialize, PartialEq, Clone)]
pub(crate) struct EntityDelta {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<Player>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<(f32, f32)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity: Option<(f32, f32)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub anim_state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<String>,
}

impl EntityDelta {
    pub fn from_entity(
        entity: specs::Entity,
        player_id: &specs::ReadStorage<Player>,
        position: &specs::ReadStorage<Position>,
        velocity: &specs::ReadStorage<Velocity>,
        direction: &specs::ReadStorage<Direction>,
        anim_state: &specs::ReadStorage<AnimState>,
        network_id: &specs::ReadStorage<NetworkId>,
        sprite: &specs::ReadStorage<Sprite>,
    ) -> Option<Self> {
        let net_id = network_id.get(entity)?.0;
        Some(Self {
            id: net_id,
            player_id: player_id.get(entity).cloned(),
            position: position.get(entity).map(|p| (p.x, p.y)),
            velocity: velocity.get(entity).map(|v| (v.x, v.y)),
            direction: direction.get(entity).map(|d| d.as_str().to_string()),
            anim_state: anim_state.get(entity).map(|a| a.0.clone()),
            sprite: sprite.get(entity).map(|s| s.0.to_string()),
        })
    }
}
