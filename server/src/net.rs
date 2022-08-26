use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

use serde_json::Error;
use whyrc_protocol::{ClientMessage, ServerMessage};

pub fn handle_client(mut stream: TcpStream) {
    // TODO: How big should our buffer be?
    // - As big as the largest variant of the Message enum?
    // - What if we read too much data from the buffer to construct the next message?
    //   - not possible stream.read will tell us how many bytes were read
    let mut buffer = [0; 128];

    while match stream.read(&mut buffer) {
        Ok(size) => {
            let message_str = std::str::from_utf8(&buffer[..size]).unwrap();
            handle_message(&mut stream, message_str);
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn handle_message(stream: &mut TcpStream, message_str: &str) {
    let message: Result<ClientMessage, Error> = serde_json::from_str(message_str);

    let response: ServerMessage = if let Ok(message) = message {
        build_response(message)
    } else {
        println!(
            "Could not parse message from {}",
            stream.peer_addr().unwrap()
        );

        ServerMessage::error_from("Could not parse that message")
    };

    let serialized_response = serde_json::to_string(&response).unwrap();

    stream.write_all(serialized_response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn build_response(message: ClientMessage) -> ServerMessage {
    match message {
        ClientMessage::Ping => ServerMessage::Pong,
    }
}
