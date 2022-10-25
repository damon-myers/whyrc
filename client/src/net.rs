use std::{
    net::TcpStream,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread::{self, JoinHandle},
};

use protocol::{ClientMessage, ServerMessage};

use self::server_connection::*;
use crate::Args;

pub use self::error::*;

mod error;
mod server_connection;

pub struct CloseableThread {
    thread_handle: Option<JoinHandle<()>>,
    should_close: Arc<AtomicBool>,
}

impl CloseableThread {
    /// Creates an atomic bool that is copied into the closure and initialized to false
    /// The closure should continually check this bool and clean itself up whenever the bool is true
    pub fn spawn<F: FnOnce(Arc<AtomicBool>) -> () + Send + 'static>(f: F) -> Self {
        let should_close = Arc::new(AtomicBool::new(false));

        let should_close_clone = should_close.clone();

        let handle = thread::spawn(move || f(should_close_clone));

        CloseableThread {
            thread_handle: Some(handle),
            should_close,
        }
    }

    pub fn close(&mut self) {
        self.should_close.swap(true, Ordering::Relaxed);

        self.thread_handle
            .take()
            .map(JoinHandle::join)
            .unwrap()
            .expect("Failed to cleanup a thread properly");
    }
}

pub struct NetworkHandles {
    // listen to this receiver for messages being sent by the server
    pub receiver: mpsc::Receiver<ServerMessage>,
    // send to this sender to send messages to the server
    pub sender: mpsc::Sender<ClientMessage>,
    receive_handle: CloseableThread,
    send_handle: CloseableThread,
}

impl Drop for NetworkHandles {
    fn drop(&mut self) {
        self.receive_handle.close();
        self.send_handle.close();
    }
}

/// Creates and setups two networking threads:
/// - one for receiving from the server,
/// - one for sending to the server
///
/// 1. Opens a TCP connection using the passed-in args
/// 2. Attempts to login to the server using the passed-in args
/// 3. If successful, sets up a thread that will read/write to the encapsulated TcpStream
///     by reading from one of the mpsc channels that gets returned
/// 4. Will notify of messages by sending them to one of the mpsc channels that gets returned
pub fn setup_network_threads(args: Args) -> Result<NetworkHandles, NetworkSetupError> {
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

    let mut connection_clone = connection.clone();

    let receive_handle = CloseableThread::spawn(move |should_close: Arc<AtomicBool>| {
        connection.listen_to_server(should_close.clone(), net_tx);
    });

    let send_handle = CloseableThread::spawn(move |should_close: Arc<AtomicBool>| {
        connection_clone.listen_to_ui(should_close.clone(), net_rx);
    });

    Ok(NetworkHandles {
        receiver: ui_rx,
        sender: ui_tx,
        receive_handle,
        send_handle,
    })
}
