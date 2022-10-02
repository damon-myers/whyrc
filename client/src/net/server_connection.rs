use std::io::{self, prelude::*, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc;

use whyrc_protocol::{ClientMessage, ServerMessage, TCP_BUFFER_SIZE};

use crate::net::error::*;

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
    pub fn try_login(
        &mut self,
        username: &str,
        server_password: &str,
    ) -> Result<(), NetworkSetupError> {
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
                ServerMessage::Error { cause } => Err(LoginError::InvalidCredentials(cause).into()),
                _ => Err(LoginError::InvalidResponse.into()),
            },
            Err(err) => Err(NetworkSetupError::TcpError(err)),
        }
    }

    fn receive_message(&mut self) -> Result<ServerMessage, io::Error> {
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
                Err(err)
            }
        }
    }

    pub fn listen(
        &mut self,
        net_rx: mpsc::Receiver<ClientMessage>,
        net_tx: mpsc::Sender<ServerMessage>,
    ) {
        todo!()
    }
}
