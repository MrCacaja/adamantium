use specs::{Entities, Entity, WriteStorage};

use crate::ecs::components::{
    id::{NetworkId, Player},
    sprite::{Sprite, SpriteType},
    transform::{AnimState, Direction, Position, Velocity},
};

pub(crate) fn create_player(
    entities: &Entities,
    network_ids: &mut WriteStorage<NetworkId>,
    player_ids: &mut WriteStorage<Player>,
    positions: &mut WriteStorage<Position>,
    sprites: &mut WriteStorage<Sprite>,
    velocities: &mut WriteStorage<Velocity>,
    directions: &mut WriteStorage<Direction>,
    anim_states: &mut WriteStorage<AnimState>,
    player_id: u32,
    network_id: u32,
    address: String,
) -> Entity {
    let player_entity = entities.create();

    player_ids
        .insert(
            player_entity,
            Player {
                id: player_id,
                address,
            },
        )
        .unwrap();

    network_ids
        .insert(player_entity, NetworkId(network_id))
        .unwrap();

    positions
        .insert(player_entity, Position::new(2., 10.))
        .unwrap();

    sprites
        .insert(player_entity, Sprite(SpriteType::Human))
        .unwrap();

    velocities
        .insert(player_entity, Velocity { x: 0.0, y: 0.0 })
        .unwrap();

    directions.insert(player_entity, Direction::Down).unwrap();

    anim_states
        .insert(player_entity, AnimState("idle".to_string()))
        .unwrap();

    player_entity
}
