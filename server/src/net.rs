use std::net::TcpListener;
use std::thread;

use rand::distributions::Alphanumeric;
use rand::prelude::*;

use crate::server::Server;

pub use connection::*;

mod connection;

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

    let server = Server::from(args);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let server_clone = server.clone();
            thread::spawn(move || {
                let mut connection = Connection::from(stream);
                // TODO: Store connections in the Server state in such a way that
                //       any thread can send data to the TcpStream in the connection
                // server_clone.add_connection(connection);
                connection.listen(server_clone)
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
