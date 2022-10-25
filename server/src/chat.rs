use std::{
    collections::BTreeMap,
    fmt::write,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use protocol::{Room, RoomList, ServerMessage};

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

            // must drop the writable lock for list_rooms to acquire a read lock
            drop(writable_rooms);

            self.list_rooms(0, None)
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

        let username_exists = writable_users
            .values()
            .any(|existing_username| &username == existing_username);

        if username_exists {
            return ServerMessage::Error {
                cause: format!("A user with name \"{}\" already exists!", username),
            };
        }

        writable_users.insert(peer_addr, username);

        ServerMessage::Ack
    }

    pub fn remove_user(&self, peer_addr: SocketAddr) {
        let mut writable_users = match self.users.write() {
            Ok(lock) => lock,
            Err(_) => {
                println!("Failed to get lock when removing user for {}", peer_addr);
                return;
            }
        };

        writable_users.remove(&peer_addr);
    }
}
