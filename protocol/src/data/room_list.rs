use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RoomListPage {
    pub room_names: Vec<String>,
    pub page: usize,
    pub total_room_count: usize,
}

impl RoomListPage {
    pub fn from(all_room_names: &Vec<&String>, page: usize, page_size: usize) -> Self {
        RoomListPage {
            room_names: util::get_page_buffer(&all_room_names, page, page_size),
            page,
            total_room_count: all_room_names.len(),
        }
    }
}
