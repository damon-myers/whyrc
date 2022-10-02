use std::io;

#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials(String),
    InvalidResponse,
}

#[derive(Debug)]
pub enum NetworkSetupError {
    LoginError(LoginError),
    DeserializationError(serde_json::Error),
    TcpError(io::Error),
}

impl From<serde_json::Error> for NetworkSetupError {
    fn from(err: serde_json::Error) -> Self {
        NetworkSetupError::DeserializationError(err)
    }
}

impl From<io::Error> for NetworkSetupError {
    fn from(err: io::Error) -> Self {
        NetworkSetupError::TcpError(err)
    }
}

impl From<LoginError> for NetworkSetupError {
    fn from(err: LoginError) -> Self {
        NetworkSetupError::LoginError(err)
    }
}
