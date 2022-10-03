use std::io;

use clap::Parser;

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
    NetworkSetupError(net::NetworkSetupError),
    LoginError(net::LoginError),
    UIError(ui::UIError),
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> Self {
        ClientError::ConnectionError(err)
    }
}

impl From<net::NetworkSetupError> for ClientError {
    fn from(err: net::NetworkSetupError) -> Self {
        ClientError::NetworkSetupError(err)
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
    let event_receiver = events::spawn_event_thread();

    let args = Args::parse();

    let net_handles = net::setup_network_threads(args)?;

    let mut ui = ui::UI::from(event_receiver, net_handles);

    ui.render_loop()?;

    Ok(())
}
