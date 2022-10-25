use protocol::Room;
use tui::widgets::ListState;

pub struct UiState {
    username: String,
    pub rooms: Vec<Room>,
    pub room_list_state: ListState,
}

impl UiState {
    pub fn from(username: &String) -> Self {
        let mut room_list_state = ListState::default();
        room_list_state.select(Some(0));

        UiState {
            username: String::from(username),
            rooms: Vec::new(),
            room_list_state,
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}
