use clap::Parser;
use std::net::TcpListener;
use std::thread;

mod net;

/// Binary application that runs a whyrc server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value = "127.0.0.1")]
    ip_address: String,

    #[clap(short, long, value_parser, default_value = "7341")]
    port: u16,
}

fn main() {
    let args = Args::parse();

    let listen_address = format!("{}:{}", args.ip_address, args.port);
    let listener = TcpListener::bind(listen_address).unwrap();

    println!(
        "Server started. Listening at {}:{}",
        args.ip_address, args.port
    );

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {
                // connection succeeded
                net::handle_client(stream)
            });
        } else {
            println!("ERROR: Connection attempted by client, but failed!");
        }
    }
}
