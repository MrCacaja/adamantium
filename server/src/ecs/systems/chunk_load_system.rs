use std::collections::HashMap;

use specs::{Entities, Join, ReadStorage, System, WriteExpect};

use crate::{
    common::events::{Action, ActionMessage, OutputEventSender, PeerType, ServerOutputEvent},
    ecs::{
        components::{id::Player, transform::Position},
        resources::world_manager::WorldManager,
    },
    log::{Log, LogLevel},
    world::chunk::ChunkCoord,
};

#[derive(Default)]
pub(crate) struct ChunkLoadSystem {
    player_chunks: HashMap<u32, ChunkCoord>,
    tick_counter: u32,
}

impl<'a> System<'a> for ChunkLoadSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, WorldManager>,
        WriteExpect<'a, OutputEventSender>,
    );

    fn run(
        &mut self,
        (entities, players, positions, mut world_manager, output_event_sender): Self::SystemData,
    ) {
        self.tick_counter += 1;

        if self.tick_counter % 10 != 0 {
            return;
        }

        let mut active_positions = Vec::new();
        let mut chunks_to_send: HashMap<ChunkCoord, Vec<String>> = HashMap::new();

        for (_ent, player, pos) in (&*entities, &players, &positions).join() {
            active_positions.push((pos.x, pos.y));

            let current_chunk = ChunkCoord::from_world_pos(pos.x, pos.y);
            let previous_chunk = self.player_chunks.get(&player.id).copied();

            if previous_chunk != Some(current_chunk) {
                Log::new(
                    LogLevel::Debug,
                    format!(
                        "Player {} moved to chunk ({}, {})",
                        player.id, current_chunk.x, current_chunk.y
                    ),
                );

                let loaded_chunks = world_manager.load_chunks_around(pos.x, pos.y);

                for chunk_coord in loaded_chunks {
                    chunks_to_send
                        .entry(chunk_coord)
                        .or_default()
                        .push(player.address.clone());
                }

                self.player_chunks.insert(player.id, current_chunk);
            }
        }

        for (coord, addresses) in chunks_to_send {
            if let Some(payload) = world_manager.serialize_chunk(coord) {
                let _ = output_event_sender.send(Box::new(ServerOutputEvent {
                    peer_ip: PeerType::Ips(addresses),
                    message: ActionMessage {
                        action: Action::SyncChunk,
                        arg: serde_json::to_string(&payload).unwrap_or_default(),
                    },
                }));
            }
        }

        if self.tick_counter % 100 == 0 {
            world_manager.unload_distant_chunks(&active_positions);
        }
    }
}
