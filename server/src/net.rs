use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use std::thread;

use rand::distributions::Alphanumeric;
use rand::prelude::*;

mod connection;
mod server;

pub use connection::*;
pub use server::*;

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

    let mut server = Server::from(args);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let stream_clone = stream.try_clone().unwrap();
            server.add_stream(stream_clone).unwrap();

            let server_clone = server.clone();
            thread::spawn(move || {
                let mut connection = Connection::from(stream, server_clone);
                connection.listen();

                println!(
                    "Connection for peer {} is being closed.",
                    connection.peer_addr
                )
            });
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
