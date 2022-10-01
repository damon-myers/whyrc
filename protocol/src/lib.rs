use serde::{Deserialize, Serialize};

pub use room_list::RoomList;

mod room_list;

pub const TCP_BUFFER_SIZE: usize = 2048;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Ping,
    CreateRoom { name: String },
    DeleteRoom { name: String },
    ListRooms { page: usize, page_size: usize },
    Login { username: String, password: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Ack,
    Pong,
    LoginSuccessful,
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
