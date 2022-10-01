use std::{
    collections::BTreeMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

mod room;
pub use room::*;
use whyrc_protocol::{RoomList, ServerMessage};

use crate::net::Server;

pub type RoomMap = BTreeMap<String, Room>;

// ip address & port -> username
// key will not be present for a user that isn't logged in
pub type UserMap = BTreeMap<SocketAddr, String>;

const DEFAULT_PAGE_SIZE: usize = 20;

#[derive(Clone)]
pub struct Chat {
    rooms: Arc<RwLock<RoomMap>>,
    users: Arc<RwLock<UserMap>>,
}

impl Chat {
    pub fn new() -> Self {
        Chat {
            rooms: Arc::new(RwLock::new(BTreeMap::new())),
            users: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn add_room(&self, room_name: String) -> ServerMessage {
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

    pub fn remove_room(&self, room_name: String) -> ServerMessage {
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

    pub fn list_rooms(&self, page: usize, page_size: Option<usize>) -> ServerMessage {
        let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE);

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

    pub fn set_username(&mut self, peer_addr: SocketAddr, username: String) -> ServerMessage {
        let mut writable_users = match self.users.write() {
            Ok(lock) => lock,
            Err(_) => return ServerMessage::error_from("Failed to get users lock"),
        };

        println!("Existing users:");
        let existing_users: Vec<&String> = writable_users.values().collect();
        println!("{:#?}", existing_users);

        let username_exists = writable_users
            .values()
            .any(|existing_username| &username == existing_username);

        println!("{:#?}", username_exists);
        if username_exists {
            return ServerMessage::Error {
                cause: format!("A user with name \"{}\" already exists!", username),
            };
        }

        writable_users.insert(peer_addr, username);

        ServerMessage::Ack
    }
}
