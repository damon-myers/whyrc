use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Ping,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Pong,
    Error { cause: String },
}

impl ServerMessage {
    pub fn error_from(cause: &str) -> Self {
        Self::Error {
            cause: String::from(cause),
        }
    }
}
