use serde::{Deserialize, Serialize};

// TODO: Remove this, we don't need pagination
// Instead just use a Vec<Room>?
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RoomList {
    pub room_names: Vec<String>,
    pub page: usize,
    pub room_count: usize,
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
