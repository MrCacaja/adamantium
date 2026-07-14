use std::collections::HashMap;

use specs::{
    storage::ComponentEvent, BitSet, Entities, ReadStorage, ReaderId, System, SystemData, World,
    WriteExpect, WriteStorage,
};

use crate::{
    common::events::{Action, ActionMessage, OutputEventSender, PeerType, ServerOutputEvent},
    ecs::components::{
        delta::EntityDelta,
        id::{NetworkId, Player},
        sprite::Sprite,
        transform::{AnimState, Direction, Position, Rotation, Scale, Velocity},
    },
};

macro_rules! read_events {
    ($storage:expr, $reader:expr, $inserted:expr, $modified:expr, $removed:expr) => {
        let events = $storage.channel().read($reader);
        for event in events {
            let _ = match event {
                ComponentEvent::Modified(id) => $modified.add(*id),
                ComponentEvent::Removed(id) => $modified.add(*id),
                ComponentEvent::Inserted(id) => $modified.add(*id),
                // ComponentEvent::Inserted(id) => $inserted.add(*id),
                // ComponentEvent::Removed(id) => $removed.add(*id),
            };
        }
    };
}

macro_rules! read_all_events {
    ($self:expr, $($component:ident => $reader:ident),* $(,)?) => {
        $(
            read_events!(
                $component,
                $self.$reader
                .as_mut()
                .expect(concat!(stringify!($reader), " not registered")),
                $self.inserted,
                $self.modified,
                $self.removed
            );
        )*
    };
}

#[derive(Default)]
pub(crate) struct TrackSystem {
    reader_player_id: Option<ReaderId<ComponentEvent>>,
    reader_position: Option<ReaderId<ComponentEvent>>,
    reader_velocity: Option<ReaderId<ComponentEvent>>,
    reader_direction: Option<ReaderId<ComponentEvent>>,
    reader_anim_state: Option<ReaderId<ComponentEvent>>,
    reader_rotation: Option<ReaderId<ComponentEvent>>,
    reader_scale: Option<ReaderId<ComponentEvent>>,
    reader_network_id: Option<ReaderId<ComponentEvent>>,
    reader_sprite: Option<ReaderId<ComponentEvent>>,
    modified: BitSet,
    last_sent: HashMap<u32, EntityDelta>,
}

impl<'a> System<'a> for TrackSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, OutputEventSender>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Direction>,
        ReadStorage<'a, AnimState>,
        ReadStorage<'a, Rotation>,
        ReadStorage<'a, Scale>,
        ReadStorage<'a, NetworkId>,
        ReadStorage<'a, Sprite>,
    );

    fn setup(&mut self, res: &mut World) {
        self.reader_player_id = Some(WriteStorage::<Player>::fetch(&res).register_reader());
        self.reader_position = Some(WriteStorage::<Position>::fetch(&res).register_reader());
        self.reader_velocity = Some(WriteStorage::<Velocity>::fetch(&res).register_reader());
        self.reader_direction = Some(WriteStorage::<Direction>::fetch(&res).register_reader());
        self.reader_anim_state = Some(WriteStorage::<AnimState>::fetch(&res).register_reader());
        self.reader_rotation = Some(WriteStorage::<Rotation>::fetch(&res).register_reader());
        self.reader_scale = Some(WriteStorage::<Scale>::fetch(&res).register_reader());
        self.reader_network_id = Some(WriteStorage::<NetworkId>::fetch(&res).register_reader());
        self.reader_sprite = Some(WriteStorage::<Sprite>::fetch(&res).register_reader());
    }

    fn run(
        &mut self,
        (
            entities,
            output_event_sender,
            player_id,
            position,
            velocity,
            direction,
            anim_state,
            rotation,
            scale,
            network_id,
            sprite,
        ): Self::SystemData,
    ) {
        self.modified.clear();

        read_all_events!(
            self,
            player_id => reader_player_id,
            position => reader_position,
            velocity => reader_velocity,
            direction => reader_direction,
            anim_state => reader_anim_state,
            rotation => reader_rotation,
            scale => reader_scale,
            network_id => reader_network_id,
            sprite => reader_sprite,
        );

        for id in &self.modified {
            let entity = entities.entity(id);

            let delta = match EntityDelta::from_entity(
                entity,
                &player_id,
                &position,
                &velocity,
                &direction,
                &anim_state,
                &rotation,
                &scale,
                &network_id,
                &sprite,
            ) {
                Some(d) => d,
                None => continue,
            };

            if self.last_sent.get(&delta.id) == Some(&delta) {
                continue;
            }

            self.last_sent.insert(delta.id, delta.clone());

            let _ = output_event_sender.send(Box::new(ServerOutputEvent {
                message: ActionMessage {
                    action: Action::SyncEntity,
                    arg: serde_json::to_string(&delta).unwrap(),
                },
                peer_ip: PeerType::Global,
            }));
        }
    }
}
