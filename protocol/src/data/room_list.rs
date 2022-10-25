use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomList {
    room_names: Vec<String>,
    page: usize,
    room_count: usize,
}

impl RoomList {
    pub fn from(room_names: Vec<&String>, page: usize, page_size: usize) -> Self {
        RoomList {
            room_names: util::get_page_buffer(&room_names, page, page_size),
            page,
            room_count: room_names.len(),
        }
    }
}
