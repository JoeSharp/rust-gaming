pub struct AdventureState {
    pub location: String,
}

impl AdventureState {
    pub fn new() -> Self {
        AdventureState {
            location: "start".into(),
        }
    }

    pub fn apply(&mut self, cmd: &str) -> String {
        match cmd.trim() {
            "look" => format!("You are at {}.", self.location),
            "north" => {
                self.location = "the northern forest".into();
                "You walk north.".into()
            }
            _ => "I don't understand that.".into(),
        }
    }
}
