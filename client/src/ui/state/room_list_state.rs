use tui::widgets::ListState;

pub struct RoomListState {
    pub is_creating_room: bool,
    pub room_names: Vec<String>,
    pub room_total_count: usize,
    pub room_list_state: ListState,
}
