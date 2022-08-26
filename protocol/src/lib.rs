use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Ping,
    Pong,
    Error { cause: String },
}

impl Message {
    pub fn error_from(cause: &str) -> Message {
        Message::Error {
            cause: String::from(cause),
        }
    }
}
