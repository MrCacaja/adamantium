pub enum Action {
    Spawn,
    Destroy,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match &self {
            Action::Spawn => "0",
            Action::Destroy => "1",
        }.parse().unwrap()
    }
}