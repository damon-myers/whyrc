pub struct Room {
    name: String,
    // messages: BTreeMap<Date, Message>
    // active_users: BTreeMap<String, SocketAddrV4>
}

impl Room {
    pub fn from(name: String) -> Self {
        Room { name }
    }
}
