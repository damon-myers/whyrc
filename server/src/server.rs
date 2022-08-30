use std::{
    collections::BTreeMap,
    net::SocketAddrV4,
    sync::{Arc, RwLock},
};

use whyrc_protocol::{ClientMessage, RoomList, ServerMessage};

use crate::net::Connection;
use crate::room::Room;

type RoomMap = BTreeMap<String, Room>;
type ConnectionMap = BTreeMap<SocketAddrV4, Connection>;

#[derive(Clone)]
pub struct Server {
    ip_address: String,
    port: u16,
    password: String,
    rooms: Arc<RwLock<RoomMap>>,
    connections: Arc<RwLock<ConnectionMap>>,
}

const DEFAULT_PAGE_SIZE: usize = 20;

impl Server {
    pub fn from(args: crate::Args) -> Self {
        Server {
            ip_address: args.ip_address,
            port: args.port,
            password: args.password.unwrap(),
            rooms: Arc::new(RwLock::new(BTreeMap::new())),
            connections: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    // TODO: shouldn't be passing the connection in here
    pub fn execute_message(
        &self,
        message: ClientMessage,
        connection: &mut Connection,
    ) -> ServerMessage {
        match message {
            ClientMessage::Ping => ServerMessage::Pong,
            ClientMessage::CreateRoom { name } => self.add_room(name),
            ClientMessage::DeleteRoom { name } => self.remove_room(name),
            ClientMessage::ListRooms { page, page_size } => self.list_rooms(page, page_size),
            ClientMessage::Login { username, password } => {
                self.login_user(connection, username, password)
            }
        }
    }

    fn add_room(&self, room_name: String) -> ServerMessage {
        let mut writable_rooms = match self.rooms.write() {
            Ok(lock) => lock,
            Err(_) => return ServerMessage::error_from("Failed to get rooms lock"),
        };

        if writable_rooms.contains_key(&room_name) {
            let cause = format!("Room with name {} already exists", room_name);
            ServerMessage::error_from(&cause)
        } else {
            writable_rooms.insert(room_name.clone(), Room::from(room_name));
            ServerMessage::Ack
        }
    }

    fn remove_room(&self, room_name: String) -> ServerMessage {
        let mut writable_rooms = match self.rooms.write() {
            Ok(lock) => lock,
            Err(_) => return ServerMessage::error_from("Failed to get rooms lock"),
        };

        if !writable_rooms.contains_key(&room_name) {
            let cause = format!("Room with name {} doesn't exist", room_name);
            ServerMessage::error_from(&cause)
        } else {
            writable_rooms.remove(&room_name);
            ServerMessage::Ack
        }
    }

    fn list_rooms(&self, page: usize, page_size: usize) -> ServerMessage {
        let readable_rooms = match self.rooms.read() {
            Ok(lock) => lock,
            Err(_) => return ServerMessage::error_from("Failed to get rooms lock"),
        };

        ServerMessage::RoomList(RoomList::from(
            readable_rooms.keys().collect(),
            page,
            page_size,
        ))
    }

    fn login_user(
        &self,
        connection: &mut Connection,
        username: String,
        password: String,
    ) -> ServerMessage {
        if password != self.password {
            return ServerMessage::error_from("Invalid password provided. Please try again.");
        }

        connection.set_username(username);

        self.list_rooms(0, DEFAULT_PAGE_SIZE)

        // TODO: Store connections in the Server state in such a way that
        //       any thread can send data to the TcpStream in the connection
        // let writable_users = match self.users.write() {
        //     Ok(lock) => lock,
        //     Err(_) => return ServerMessage::error_from("Failed to get users lock"),
        // };

        // writable_users.insert(self.active_stream.)
    }
}
