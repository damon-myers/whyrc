use std::io;

use clap::Parser;
use net::try_connect;

mod events;
mod net;
mod ui;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short = 's', long, value_parser, default_value = "127.0.0.1")]
    server_address: String,

    #[clap(short = 'P', long, value_parser, default_value = "7341")]
    port: u16,

    #[clap(short, long, value_parser)]
    password: String,

    #[clap(short, long, value_parser)]
    username: String,
}

#[derive(Debug)]
pub enum ClientError {
    ConnectionError(io::Error),
    LoginError(net::LoginError),
    UIError(ui::UIError),
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> Self {
        ClientError::ConnectionError(err)
    }
}

impl From<net::LoginError> for ClientError {
    fn from(err: net::LoginError) -> Self {
        ClientError::LoginError(err)
    }
}

impl From<ui::UIError> for ClientError {
    fn from(err: ui::UIError) -> Self {
        ClientError::UIError(err)
    }
}

fn main() -> Result<(), ClientError> {
    let receiver = events::spawn_event_thread();

    let args = Args::parse();

    let mut server_conn = try_connect(&args.server_address, args.port)?;

    server_conn.try_login(&args.username, &args.password)?;

    println!("Successfully logged in as {}", args.username);

    let mut ui = ui::UI::from(receiver);

    ui.render_loop()?;

    Ok(())
}
