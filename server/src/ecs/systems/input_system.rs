use serde::Deserialize;
use specs::{Entities, Join, System, Write, WriteExpect, WriteStorage};

use crate::{
    common::events::{
        Action, ActionMessage, InputEventReceiver, OutputEventSender, PeerEvent, PeerType,
        ServerOutputEvent,
    },
    ecs::{
        components::{
            delta::EntityDelta,
            id::{NetworkId, Player},
            player_name::PlayerName,
            sprite::Sprite,
            transform::{AnimState, Direction, Position, Velocity},
        },
        resources::counters::{NetworkIdCounter, PlayerIdCounter},
        utils::templates::player::create_player,
    },
};

const SPEED: f32 = 50.;
const CHAT_RADIUS: f32 = 300.0;

#[derive(Deserialize, Debug)]
struct InputMessage {
    input_type: String,
    actor_id: String,
    args: String,
}

pub(crate) struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, InputEventReceiver>,
        WriteExpect<'a, OutputEventSender>,
        Write<'a, PlayerIdCounter>,
        Write<'a, NetworkIdCounter>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, NetworkId>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Direction>,
        WriteStorage<'a, AnimState>,
        WriteStorage<'a, PlayerName>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut event_receiver,
            output_event_senders,
            mut player_id_counter,
            mut network_id_counter,
            mut positions,
            mut player_ids,
            mut network_ids,
            mut sprites,
            mut velocities,
            mut directions,
            mut anim_states,
            mut player_names,
        ): Self::SystemData,
    ) {
        while let Ok(event) = event_receiver.try_recv() {
            let event = event.as_ref();
            match *event {
                PeerEvent::Input(ref input_event) => {
                    println!(
                        "Received input from {}: {}",
                        input_event.peer_socket, input_event.input
                    );

                    let msg: InputMessage = match serde_json::from_str(&input_event.input) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("Failed to parse input: {}", e);
                            continue;
                        }
                    };

                    let actor_id: u32 = match msg.actor_id.parse() {
                        Ok(id) => id,
                        Err(_) => continue,
                    };

                    let peer_addr = input_event.peer_socket.to_string();

                    match msg.input_type.as_str() {
                        "Move" => {
                            for (_ent, player, vel) in
                                (&*entities, &player_ids, &mut velocities).join()
                            {
                                if player.id == actor_id && player.address == peer_addr {
                                    if msg.args == "stop" {
                                        vel.x = 0.;
                                        vel.y = 0.;
                                    } else {
                                        let mut dir = (0.0f32, 0.0f32);
                                        for code in msg.args.split(',') {
                                            match code.trim() {
                                                "0" => dir.1 -= 1.,
                                                "1" => dir.0 -= 1.,
                                                "2" => dir.1 += 1.,
                                                "3" => dir.0 += 1.,
                                                _ => {}
                                            }
                                        }
                                        let len = (dir.0 * dir.0 + dir.1 * dir.1).sqrt();
                                        if len > 0. {
                                            vel.x = (dir.0 / len) * SPEED;
                                            vel.y = (dir.1 / len) * SPEED;
                                        } else {
                                            vel.x = 0.;
                                            vel.y = 0.;
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                        "SetName" => {
                            for (ent, player) in (&*entities, &player_ids).join() {
                                if player.id == actor_id && player.address == peer_addr {
                                    player_names
                                        .insert(ent, PlayerName(msg.args.clone().to_string()))
                                        .unwrap();
                                    break;
                                }
                            }
                        }
                        "Chat" => {
                            let mut sender_pos = None;
                            let mut sender_name = String::new();

                            for (_ent, player, pos, name) in
                                (&*entities, &player_ids, &positions, &player_names).join()
                            {
                                if player.id == actor_id && player.address == peer_addr {
                                    sender_pos = Some((pos.x, pos.y));
                                    sender_name = name.0.clone();
                                    break;
                                }
                            }

                            let sender_pos = match sender_pos {
                                Some(p) => p,
                                None => continue,
                            };

                            let chat_payload = serde_json::json!({
                                "sender": sender_name,
                                "message": msg.args
                            })
                            .to_string();
                            let mut nearby_ips = Vec::new();
                            for (_ent, player, pos) in (&*entities, &player_ids, &positions).join()
                            {
                                let dx = pos.x - sender_pos.0;
                                let dy = pos.y - sender_pos.1;
                                let dist = (dx * dx + dy * dy).sqrt();
                                if dist <= CHAT_RADIUS {
                                    nearby_ips.push(player.address.clone());
                                }
                            }

                            let _ = output_event_senders.send(Box::new(ServerOutputEvent {
                                peer_ip: PeerType::Ips(nearby_ips),
                                message: ActionMessage {
                                    action: Action::Chat,
                                    arg: chat_payload.clone(),
                                },
                            }));
                        }
                        _ => {}
                    }
                }
                PeerEvent::Connected(ref connected_event) => {
                    let player_id = player_id_counter.0 .0.next();
                    let network_id = network_id_counter.0 .0.next();
                    let new_addr = connected_event.peer_socket.to_string();

                    create_player(
                        &entities,
                        &mut network_ids,
                        &mut player_ids,
                        &mut positions,
                        &mut sprites,
                        &mut velocities,
                        &mut directions,
                        &mut anim_states,
                        player_id,
                        network_id,
                        new_addr.clone(),
                    );

                    output_event_senders
                        .send(Box::new(ServerOutputEvent {
                            peer_ip: PeerType::Ip(new_addr.clone()),
                            message: ActionMessage {
                                action: Action::SyncId,
                                arg: player_id.to_string(),
                            },
                        }))
                        .unwrap_or_else(|e| eprintln!("Failed to send output event: {}", e));

                    // TODO: send the entities based on the new player position and the chunk system, not all entities
                    for (ent, net_id) in (&*entities, &network_ids).join() {
                        let delta = EntityDelta {
                            id: net_id.0,
                            player_id: player_ids.get(ent).cloned(),
                            position: positions.get(ent).map(|p| (p.x, p.y)),
                            velocity: velocities.get(ent).map(|v| (v.x, v.y)),
                            direction: directions.get(ent).map(|d| d.as_str().to_string()),
                            anim_state: anim_states.get(ent).map(|a| a.0.clone()),
                            sprite: sprites.get(ent).map(|s| s.0.to_string()),
                        };

                        let _ = output_event_senders.send(Box::new(ServerOutputEvent {
                            peer_ip: PeerType::Ip(new_addr.clone()),
                            message: ActionMessage {
                                action: Action::SyncEntity,
                                arg: serde_json::to_string(&delta).unwrap(),
                            },
                        }));
                    }
                }
                PeerEvent::Disconnected(ref disconnected_event) => {
                    output_event_senders
                        .send(Box::new(ServerOutputEvent {
                            peer_ip: PeerType::Ip(disconnected_event.peer_socket.to_string()),
                            message: ActionMessage {
                                action: Action::Disconnect,
                                arg: disconnected_event.peer_socket.to_string(),
                            },
                        }))
                        .unwrap_or_else(|e| eprintln!("Failed to send output event: {}", e));
                    for (ent, player) in (&*entities, &mut player_ids).join() {
                        if player.address == disconnected_event.peer_socket.to_string() {
                            entities.delete(ent).unwrap();
                            break;
                        }
                    }
                }
            }
        }
    }
}
