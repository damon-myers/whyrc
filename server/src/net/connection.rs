use std::{
    io::prelude::*,
    net::{Shutdown, SocketAddr, TcpStream},
};

use whyrc_protocol::{ClientMessage, ServerMessage};

use crate::net::Server;

/// Thin wrapper around a TcpStream that forwards messages to the Server
pub struct Connection {
    active_stream: TcpStream,
    peer_addr: SocketAddr,
    server: Server,
}

impl Connection {
    pub fn from(stream: TcpStream, server_clone: Server) -> Self {
        let peer_addr = stream.peer_addr().unwrap();

        Connection {
            active_stream: stream,
            server: server_clone,
            peer_addr,
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

    fn handle_message(&mut self, message_str: &str) {
        let message: Result<ClientMessage, serde_json::Error> = serde_json::from_str(message_str);

        let response: ServerMessage = if let Ok(message) = message {
            self.server.execute_command(self.peer_addr, message)
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
}