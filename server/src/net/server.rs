use std::{
    collections::BTreeMap,
    net::{SocketAddr, TcpStream},
    sync::{Arc, RwLock},
};

use whyrc_protocol::{ClientMessage, ServerMessage};

use crate::chat::Chat;

type StreamMap = BTreeMap<SocketAddr, TcpStream>;

#[derive(Clone)]
pub struct Server {
    ip_address: String,
    port: u16,
    password: String,
    tcp_streams: Arc<RwLock<StreamMap>>,
    chat: Chat,
}

impl Server {
    pub fn from(args: crate::Args) -> Self {
        Server {
            ip_address: args.ip_address,
            port: args.port,
            password: args.password.unwrap(),
            tcp_streams: Arc::new(RwLock::new(BTreeMap::new())),
            chat: Chat::new(),
        }
    }

    pub fn execute_command(&mut self, message: ClientMessage) -> ServerMessage {
        match message {
            ClientMessage::Ping => ServerMessage::Pong,
            ClientMessage::CreateRoom { name } => self.chat.add_room(name),
            ClientMessage::DeleteRoom { name } => self.chat.remove_room(name),
            ClientMessage::ListRooms { page, page_size } => {
                self.chat.list_rooms(page, Some(page_size))
            }
            ClientMessage::Login { .. } => {
                ServerMessage::error_from("Got a login message where server shouldn't have")
            }
        }
    }

    pub fn add_stream(&mut self, tcp_stream: TcpStream) -> Result<(), ()> {
        let peer_addr = tcp_stream.peer_addr().unwrap();

        let mut writeable_conns = match self.tcp_streams.write() {
            Ok(lock) => lock,
            Err(_) => return Err(()),
        };

        writeable_conns.insert(peer_addr, tcp_stream);

        Ok(())
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }
}
