use protocol::Room;

pub struct UiState {
    username: String,
    pub rooms: Vec<Room>,
}

impl UiState {
    pub fn from(username: &String) -> Self {
        UiState {
            username: String::from(username),
            rooms: Vec::new(),
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}
