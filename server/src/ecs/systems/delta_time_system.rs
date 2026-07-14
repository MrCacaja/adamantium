use specs::{System, Write};
use std::time::Instant;

use crate::ecs::resources::delta::DeltaTime;

pub(crate) struct DeltaTimeSystem {
    last_tick: Instant,
}

impl Default for DeltaTimeSystem {
    fn default() -> Self {
        Self {
            last_tick: Instant::now(),
        }
    }
}

impl<'a> System<'a> for DeltaTimeSystem {
    type SystemData = Write<'a, DeltaTime>;

    fn run(&mut self, mut dt: Self::SystemData) {
        let now = Instant::now();
        dt.0 = now.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = now;
    }
}
