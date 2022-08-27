use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use rand::distributions::Alphanumeric;
use rand::prelude::*;
use serde_json::Error;

use server::Server;
use whyrc_protocol::{ClientMessage, ServerMessage};

mod server;

pub fn handle_client(server: Arc<Server>, mut stream: TcpStream) {
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

pub fn start_server(mut args: crate::Args) {
    if args.password.is_none() {
        let password = generate_password();
        println!(
            "No password was provided. Generated password is: {}",
            password
        );

        args.password = Some(password);
    }

    let listen_address = format!("{}:{}", args.ip_address, args.port);
    let listener = TcpListener::bind(listen_address).unwrap();

    println!(
        "Server started. Listening at {}:{}",
        args.ip_address, args.port
    );

    let server = Arc::new(Server::from(args));

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let server_ref = server.clone();
            thread::spawn(move || handle_client(server_ref, stream));
        } else {
            println!("ERROR: Connection attempted by client, but failed!");
        }
    }
}

fn generate_password() -> String {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    password
}
