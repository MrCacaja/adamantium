pub(crate) struct Counter {
    pub(crate) counter: u32,
}

impl Counter {
    pub(crate) fn new() -> Self {
        Self { counter: 0 }
    }

    pub(crate) fn next(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub(crate) struct IdCounter(pub(crate) Counter);

#[derive(Default)]
pub(crate) struct PlayerIdCounter(pub(crate) IdCounter);
#[derive(Default)]
pub(crate) struct NetworkIdCounter(pub(crate) IdCounter);
