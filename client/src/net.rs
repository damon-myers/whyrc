use std::{
    io::{self, prelude::*, Write},
    net::{Shutdown, TcpStream},
};

use whyrc_protocol::{ClientMessage, ServerMessage, TCP_BUFFER_SIZE};

pub use self::error::*;

mod error;

pub struct ServerConnection {
    stream: TcpStream,
}

impl ServerConnection {
    pub fn from(stream: TcpStream) -> Self {
        ServerConnection { stream }
    }

    /// Attempt to login to the server given a username and server_password
    /// Blocks until a response is received from the server
    /// Can potentially fail due to:
    /// - invalid password
    /// - already claimed username
    /// - network issues
    pub fn try_login(&mut self, username: &str, server_password: &str) -> Result<(), LoginError> {
        println!("Attempting to login to server...");

        let login_message = ClientMessage::Login {
            username: String::from(username),
            password: String::from(server_password),
        };

        let serialized_login = serde_json::to_string(&login_message).unwrap();

        self.stream.write_all(serialized_login.as_bytes())?;

        let response = self.receive_message();

        match response {
            Ok(message) => match message {
                ServerMessage::LoginSuccessful => Ok(()),
                ServerMessage::Error { cause } => Err(LoginError::InvalidCredentials(cause)),
                _ => Err(LoginError::InvalidResponse),
            },
            Err(err) => Err(LoginError::ConnectionError(err)),
        }
    }

    pub fn receive_message(&mut self) -> Result<ServerMessage, ReceiveMessageError> {
        let mut buffer = [0; TCP_BUFFER_SIZE];

        match self.stream.read(&mut buffer) {
            Ok(size) => {
                let message_str = std::str::from_utf8(&buffer[..size]).unwrap();
                let message = serde_json::from_str(message_str)?;

                Ok(message)
            }
            Err(err) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    self.stream.peer_addr().unwrap()
                );
                self.stream.shutdown(Shutdown::Both).unwrap();
                Err(ReceiveMessageError::TcpError(err))
            }
        }
    }
}

pub fn try_connect(host: &str, port: u16) -> io::Result<ServerConnection> {
    let stream = TcpStream::connect(format!("{}:{}", host, port))?;

    Ok(ServerConnection::from(stream))
}
