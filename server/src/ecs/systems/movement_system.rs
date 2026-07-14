use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::ecs::components::transform::{Position, Velocity};
use crate::ecs::resources::delta::DeltaTime;

pub(crate) struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (dt, velocities, mut positions): Self::SystemData) {
        for (vel, pos) in (&velocities, &mut positions).join() {
            if vel.x == 0.0 && vel.y == 0.0 {
                continue;
            }
            pos.x += vel.x * dt.0;
            pos.y += vel.y * dt.0;
        }
    }
}
