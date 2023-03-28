pub enum Action {
    Spawn,
    Destroy,
    SendState
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match &self {
            Action::Spawn => "0",
            Action::Destroy => "1",
            Action::SendState => "2",
        }.parse().unwrap()
    }
}