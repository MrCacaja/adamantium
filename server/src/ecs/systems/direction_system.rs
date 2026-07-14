use specs::{Join, ReadStorage, System, WriteStorage};

use crate::ecs::components::transform::{AnimState, Direction, Velocity};

pub(crate) struct DirectionSystem;

impl<'a> System<'a> for DirectionSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Direction>,
        WriteStorage<'a, AnimState>,
    );

    fn run(&mut self, (velocities, mut directions, mut anim_states): Self::SystemData) {
        for (vel, dir, anim) in (&velocities, &mut directions, &mut anim_states).join() {
            let moving = vel.x != 0.0 || vel.y != 0.0;

            let new_anim = if moving { "walk" } else { "idle" };
            if anim.0 != new_anim {
                anim.0 = new_anim.to_string();
            }

            if !moving {
                continue;
            }

            if vel.y.abs() >= vel.x.abs() {
                if vel.y < 0.0 {
                    *dir = Direction::Up;
                } else {
                    *dir = Direction::Down;
                }
            } else {
                if vel.x < 0.0 {
                    *dir = Direction::Left;
                } else {
                    *dir = Direction::Right;
                }
            }
        }
    }
}
