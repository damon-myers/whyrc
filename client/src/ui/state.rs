use protocol::{Room, RoomList};
use tui::widgets::ListState;

pub struct UiState {
    username: String,
    pub room_data: Vec<Room>,
    pub room_list: RoomList,
    pub room_list_state: ListState,
}

impl UiState {
    pub fn from(username: &String) -> Self {
        let mut room_list_state = ListState::default();
        room_list_state.select(Some(0));

        UiState {
            username: String::from(username),
            room_data: Vec::new(),
            room_list: RoomList::default(),
            room_list_state,
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}
