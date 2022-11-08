use protocol::Room;
use tui::widgets::ListState;

use self::room_list_state::RoomListState;

mod room_list_state;

pub struct UiState {
    username: String,
    pub room_data: Vec<Room>,
    pub room_list: RoomListState,
}

impl UiState {
    pub fn from(username: &String) -> Self {
        let mut room_list_state = ListState::default();
        room_list_state.select(Some(0));

        UiState {
            username: String::from(username),
            room_data: Vec::new(),
            room_list: RoomListState {
                is_creating_room: false,
                room_names: Vec::new(),
                room_total_count: 0,
                room_list_state,
            },
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}
