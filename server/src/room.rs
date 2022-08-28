pub struct Room {
    name: String,
    // messages: BTreeMap<Date, Message>
    // active_users: BTreeMap<
}

impl Room {
    pub fn from(name: String) -> Self {
        Room { name }
    }
}
