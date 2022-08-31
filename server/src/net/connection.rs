use std::{
    io::prelude::*,
    net::{Shutdown, TcpStream},
};

use whyrc_protocol::{ClientMessage, ServerMessage};

use crate::net::Server;

pub struct Connection {
    active_stream: TcpStream,
    server: Server,
    username: Option<String>, // if not logged in, will be None
}

impl Connection {
    pub fn from(stream: TcpStream, server_clone: Server) -> Self {
        Connection {
            active_stream: stream,
            server: server_clone,
            username: None,
        }
    }

    /// Reads the wrapped TcpStream for messages and responds with ServerMessages
    pub fn listen(&mut self) {
        // TODO: How big should our buffer be?
        // - As big as the largest variant of the Message enum?
        // - What if we read too much data from the buffer to construct the next message?
        //   - not possible stream.read will tell us how many bytes were read
        let mut buffer = [0; 128];

        while match self.active_stream.read(&mut buffer) {
            Ok(size) => {
                let message_str = std::str::from_utf8(&buffer[..size]).unwrap();
                self.handle_message(message_str);
                true
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    self.active_stream.peer_addr().unwrap()
                );
                self.active_stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
    }

    pub fn set_username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);

        self
    }

    fn handle_message(&mut self, message_str: &str) {
        let message: Result<ClientMessage, serde_json::Error> = serde_json::from_str(message_str);

        let response: ServerMessage = if let Ok(message) = message {
            match message {
                ClientMessage::Login { username, password } => self.login_user(username, password),
                _ => self.server.execute_command(message),
            }
        } else {
            println!(
                "Could not parse message from {}",
                self.active_stream.peer_addr().unwrap()
            );

            ServerMessage::error_from("Could not parse that message")
        };

        let serialized_response = serde_json::to_string(&response).unwrap();

        self.active_stream
            .write_all(serialized_response.as_bytes())
            .unwrap();
        self.active_stream.flush().unwrap();
    }

    fn login_user(&mut self, username: String, password: String) -> ServerMessage {
        if password != self.server.get_password() {
            return ServerMessage::error_from("Invalid password provided. Please try again.");
        }

        self.set_username(username);

        ServerMessage::Ack
    }
}
