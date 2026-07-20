mod common;
mod ecs;
mod log;
mod networking;

use crate::common::events::{create_input_event_channel, create_output_event_channel, Action};
use crate::ecs::resources::delta::DeltaTime;
use crate::ecs::resources::networking::PeerMap;
use crate::ecs::systems::delta_time_system::DeltaTimeSystem;
use crate::ecs::systems::direction_system::DirectionSystem;
use crate::ecs::systems::input_system::InputSystem;
use crate::ecs::systems::movement_system::MovementSystem;
use crate::log::{Log, LogLevel};
use crate::networking::accept_connection;
use common::events::PeerType;
use ecs::components::player_name::PlayerName;
use ecs::components::transform::{Rotation, Scale};
use ecs::systems::track_system::TrackSystem;
use futures_util::lock::Mutex;
use futures_util::SinkExt;
use specs::prelude::*;
use std::string::ToString;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9002")
        .await
        .expect("Can't listen");
    Log::new(
        LogLevel::Info,
        "Listening on: ".to_string() + &*"127.0.0.1:9002".to_string(),
    );

    let (input_event_sender, input_event_receiver) = create_input_event_channel();
    let (output_event_sender, output_event_receiver) = create_output_event_channel();
    let peer_map = Arc::new(Mutex::new(PeerMap::default()));

    let networking_thread = tokio::spawn({
        let peer_map = peer_map.clone();
        async move {
            while let Ok((stream, _)) = listener.accept().await {
                let peer = stream
                    .peer_addr()
                    .expect("connected streams should have a peer address");
                Log::new(
                    LogLevel::Info,
                    "Peer address: ".to_string() + &*peer.to_string(),
                );

                tokio::spawn(accept_connection(
                    stream,
                    input_event_sender.clone(),
                    peer_map.clone(),
                ));
                // TODO: sleep configurável ( e melhor planejado)
                sleep(Duration::from_millis(100));
            }
        }
    });

    let ecs_thread = tokio::spawn(async move {
        let mut world = World::new();
        world.insert(input_event_receiver);
        world.insert(output_event_sender);
        world.insert(DeltaTime::default());
        //TODO: remover esses register quando comecar a usar
        world.register::<Rotation>();
        world.register::<Scale>();
        let mut dispatcher = DispatcherBuilder::new()
            .with(DeltaTimeSystem::default(), "delta_time_system", &[])
            .with(InputSystem, "input_system", &[])
            .with(
                MovementSystem,
                "movement_system",
                &["input_system", "delta_time_system"],
            )
            .with(DirectionSystem, "direction_system", &["movement_system"])
            .with(
                TrackSystem::default(),
                "track_system",
                &["direction_system"],
            )
            .build();
        dispatcher.setup(&mut world);
        loop {
            dispatcher.dispatch(&mut world);
            world.maintain();
            // TODO: sleep configurável ( e melhor planejado)
            sleep(Duration::from_millis(100));
        }
    });

    let output_thread = tokio::spawn(async move {
        let mut event_receiver = output_event_receiver;
        Log::new(LogLevel::Info, "Output thread started".to_string());
        loop {
            while let Ok(event) = event_receiver.try_recv() {
                match event.peer_ip {
                    // TODO: remover redundancias
                    PeerType::Ip(ip) => {
                        Log::new(
                            LogLevel::Info,
                            "Received output event for peer {}: {}".to_string()
                        // + &*event.peer_ip
                        + " - "
                        + &*event.message.to_string(),
                        );
                        let peer_map = peer_map.clone();
                        let mut peer_map = peer_map.lock().await;

                        if event.message.action.to_string() == Action::Disconnect.to_string() {
                            peer_map.0.remove(&ip);
                            break;
                        }

                        Log::new(
                            LogLevel::Info,
                            "Sending message to {}: {}".to_string()
                                + &*ip
                                + " - "
                                + &*event.message.to_string(),
                        );

                        if let Some(stream) = peer_map.0.get_mut(&ip) {
                            let mut stream = stream.lock().await;
                            stream
                                .send(event.message.to_string().into())
                                .await
                                .unwrap_or_else(|e| {
                                    eprintln!("Failed to send message to {}: {}", ip, e)
                                });
                        } else {
                            Log::new(
                                LogLevel::Error,
                                "Failed to send message to {}: peer not found".to_string() + &*ip,
                            );
                        }
                    }
                    PeerType::Ips(ips) => {
                        for ip in ips {
                            Log::new(
                                LogLevel::Info,
                                "Received output event for peer {}: {}".to_string()
                                    + &*ip
                                    + " - "
                                    + &*event.message.to_string(),
                            );
                            let peer_map = peer_map.clone();
                            let mut peer_map = peer_map.lock().await;
                            Log::new(
                                LogLevel::Info,
                                "Sending message to {}: {}".to_string()
                                    + &*ip
                                    + " - "
                                    + &*event.message.to_string(),
                            );
                            if let Some(stream) = peer_map.0.get_mut(&ip) {
                                let mut stream = stream.lock().await;
                                stream
                                    .send(event.message.to_string().into())
                                    .await
                                    .unwrap_or_else(|e| {
                                        eprintln!("Failed to send message to {}: {}", ip, e)
                                    });
                            } else {
                                Log::new(
                                    LogLevel::Error,
                                    "Failed to send message to {}: peer not found".to_string()
                                        + &*ip,
                                );
                            }
                        }
                    }
                    PeerType::Global => {
                        Log::new(
                            LogLevel::Info,
                            "Received output event for all peers: {}".to_string()
                                + " - "
                                + &*event.message.to_string(),
                        );
                        let peer_map = peer_map.clone();
                        let mut peer_map = peer_map.lock().await;
                        for (ip, stream) in peer_map.0.iter_mut() {
                            Log::new(
                                LogLevel::Info,
                                "Sending message to {}: {}".to_string()
                                    + &*ip
                                    + " - "
                                    + &*event.message.to_string(),
                            );
                            let mut stream = stream.lock().await;
                            stream
                                .send(event.message.to_string().into())
                                .await
                                .unwrap_or_else(|e| {
                                    eprintln!("Failed to send message to {}: {}", ip, e)
                                });
                        }
                    }
                };
            }
            // TODO: sleep configurável ( e melhor planejado)
            sleep(Duration::from_millis(100));
        }
    });

    let _ = tokio::join!(networking_thread, output_thread, ecs_thread);
}
