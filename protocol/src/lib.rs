use serde::{Deserialize, Serialize};

pub use room_list::RoomList;

mod room_list;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Ping,
    CreateRoom { name: String },
    DeleteRoom { name: String },
    ListRooms { page: usize, page_size: usize },
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
