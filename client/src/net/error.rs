use std::io;

#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials(String),
    InvalidResponse,
    ConnectionError(ReceiveMessageError),
}

impl From<io::Error> for LoginError {
    fn from(err: io::Error) -> Self {
        LoginError::ConnectionError(ReceiveMessageError::from(err))
    }
}

#[derive(Debug)]
pub enum ReceiveMessageError {
    DeserializationError(serde_json::Error),
    TcpError(io::Error),
}

impl From<serde_json::Error> for ReceiveMessageError {
    fn from(err: serde_json::Error) -> Self {
        ReceiveMessageError::DeserializationError(err)
    }
}

impl From<io::Error> for ReceiveMessageError {
    fn from(err: io::Error) -> Self {
        ReceiveMessageError::TcpError(err)
    }
}
