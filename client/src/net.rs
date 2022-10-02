use std::{net::TcpStream, sync::mpsc, thread, thread::JoinHandle};

use whyrc_protocol::{ClientMessage, ServerMessage};

use self::server_connection::*;
use crate::Args;

pub use self::error::*;

mod error;
mod server_connection;

pub struct NetworkHandles {
    // listen to this receiver for messages being sent by the server
    pub receiver: mpsc::Receiver<ServerMessage>,
    // send to this sender to send messages to the server
    pub sender: mpsc::Sender<ClientMessage>,
    pub thread_handle: JoinHandle<()>,
}

/// Creates and setups a networking thread
///
/// 1. Opens a TCP connection using the passed-in args
/// 2. Attempts to login to the server using the passed-in args
/// 3. If successful, sets up a thread that will read/write to the encapsulated TcpStream
///     by reading from one of the mpsc channels that gets returned
/// 4. Will notify of messages by sending them to one of the mpsc channels that gets returned
pub fn setup_network_thread(args: Args) -> Result<NetworkHandles, NetworkSetupError> {
    let Args {
        username,
        password,
        server_address,
        port,
    } = args;

    let stream = TcpStream::connect(format!("{}:{}", server_address, port))?;

    let mut connection = ServerConnection::from(stream);

    connection.try_login(&username, &password)?;

    println!("Successfully logged in as {}", username);

    // ui will send to ui_tx, network thread will read from net_rx
    let (ui_tx, net_rx) = mpsc::channel();

    // net will send to net_tx, ui will read from ui_rx
    let (net_tx, ui_rx) = mpsc::channel();

    let thread_handle = thread::spawn(move || {
        connection.listen(net_rx, net_tx);
    });

    Ok(NetworkHandles {
        receiver: ui_rx,
        sender: ui_tx,
        thread_handle,
    })
}
