use serde::{Deserialize, Serialize};

mod data;

pub use data::*;

pub const TCP_BUFFER_SIZE: usize = 2048;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Ping,
    CreateRoom { name: String },
    DeleteRoom { name: String },
    ListAllRooms { page_size: usize },
    Login { username: String, password: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Ack,
    Pong,
    LoginSuccessful,
    RoomList(RoomListPage),
    Error { cause: String },
}

impl ServerMessage {
    pub fn error_from(cause: &str) -> Self {
        Self::Error {
            cause: String::from(cause),
        }
    }
}
