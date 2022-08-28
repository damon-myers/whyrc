use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomList {
    room_names: Vec<String>,
    page: u32,
    room_count: usize,
}

impl RoomList {
    pub fn from(room_names: Vec<&String>, page: u32, page_size: u32) -> Self {
        let page_size = page_size as usize;
        let offset = page as usize * page_size;

        let room_name_buffer = if room_names.len() >= page_size {
            &room_names[..]
        } else if offset + page_size > room_names.len() {
            &room_names[offset..room_names.len()]
        } else {
            &room_names[offset..offset + page_size]
        };

        RoomList {
            room_names: room_name_buffer
                .iter()
                .map(|name| String::from(*name))
                .collect(),
            page,
            room_count: room_names.len(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Ping,
    CreateRoom { name: String },
    ListRooms { page: u32, page_size: u32 },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Ack,
    Pong,
    RoomList(RoomList),
    Error { cause: String },
}

impl ServerMessage {
    pub fn error_from(cause: &str) -> Self {
        Self::Error {
            cause: String::from(cause),
        }
    }
}
