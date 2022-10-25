use std::io::ErrorKind::{TimedOut, WouldBlock};
use std::io::{self, prelude::*, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use protocol::{ClientMessage, ServerMessage, TCP_BUFFER_SIZE};

use crate::net::error::*;

const RECEIVE_TIMEOUT: Duration = Duration::from_millis(200);

pub struct ServerConnection {
    stream: TcpStream,
}

impl Clone for ServerConnection {
    fn clone(&self) -> Self {
        let stream_clone = self.stream.try_clone().unwrap();
        Self {
            stream: stream_clone,
        }
    }
}

impl ServerConnection {
    pub fn from(stream: TcpStream) -> Self {
        stream.set_read_timeout(Some(RECEIVE_TIMEOUT)).unwrap();

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

        let mut ticks = 0;
        let response: Result<ServerMessage, NetworkSetupError> = loop {
            let response = self.receive_server_message();

            // time out after waiting for 10 seconds
            if RECEIVE_TIMEOUT.as_secs() * ticks >= 10u64 {
                return Err(NetworkSetupError::LoginError(LoginError::ServerTimeout));
            }

            let response = match response {
                Ok(Some(message)) => Ok(message),
                Ok(None) => {
                    ticks += 1;
                    continue;
                }
                Err(err) => Err(NetworkSetupError::TcpError(err)),
            };

            break response;
        };

        match response {
            Ok(ServerMessage::LoginSuccessful) => Ok(()),
            Ok(ServerMessage::Error { cause }) => Err(LoginError::InvalidCredentials(cause).into()),
            Ok(_) => Err(LoginError::InvalidResponse.into()),
            Err(err) => Err(err),
        }
    }

    /// Receives messages from the server, forwards them to the ui via net_tx
    pub fn listen_to_server(
        &mut self,
        should_close: Arc<AtomicBool>,
        net_tx: mpsc::Sender<ServerMessage>,
    ) {
        loop {
            if should_close.load(Ordering::Relaxed) {
                break;
            }

            let message = self.receive_server_message();

            match message {
                Ok(Some(message)) => {
                    net_tx.send(message).unwrap();
                }
                Ok(None) | Err(_) => continue,
            }
        }
    }

    /// Receives messages from the UI, forwards them to the server via the encapsulated TcpStream
    pub fn listen_to_ui(
        &mut self,
        should_close: Arc<AtomicBool>,
        net_rx: mpsc::Receiver<ClientMessage>,
    ) {
        loop {
            if should_close.load(Ordering::Relaxed) {
                break;
            }

            let receive_result = net_rx.recv_timeout(RECEIVE_TIMEOUT);

            match receive_result {
                Ok(message) => self.send_to_server(message),
                Err(_) => continue,
            }
        }
    }

    fn receive_server_message(&mut self) -> Result<Option<ServerMessage>, io::Error> {
        let mut buffer = [0; TCP_BUFFER_SIZE];

        // Still uses RECEIVE_TIMEOUT, but it is configured in ServerConnection::from
        match self.stream.read(&mut buffer) {
            Ok(size) => {
                let message_str = std::str::from_utf8(&buffer[..size]).unwrap();
                let message = serde_json::from_str(message_str)?;

                Ok(message)
            }
            Err(ref err) if err.kind() == TimedOut || err.kind() == WouldBlock => {
                // just timed out waiting on a message. nothing to be done
                Ok(None)
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

    fn send_to_server(&mut self, message: ClientMessage) {
        let serialized_response = serde_json::to_string(&message).unwrap();

        self.stream
            .write_all(serialized_response.as_bytes())
            .unwrap();

        self.stream.flush().unwrap();
    }
}
