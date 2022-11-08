use std::{
    collections::BTreeMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use protocol::{Room, RoomListPage, ServerMessage};

pub type RoomMap = BTreeMap<String, Room>;

// ip address & port -> username
// key will not be present for a user that isn't logged in
pub type UserMap = BTreeMap<SocketAddr, String>;

const DEFAULT_PAGE_SIZE: usize = 40;
const MAX_NUM_ROOMS: usize = 200;

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

    pub fn add_room(&self, room_name: String) -> Vec<ServerMessage> {
        let mut writable_rooms = match self.rooms.write() {
            Ok(lock) => lock,
            Err(_) => return vec![ServerMessage::error_from("Failed to get rooms lock")],
        };

        if writable_rooms.contains_key(&room_name) {
            let cause = format!("Room with name {} already exists", room_name);
            vec![ServerMessage::error_from(&cause)]
        } else if writable_rooms.len() >= MAX_NUM_ROOMS {
            let cause = format!(
                "{} rooms already exist. Please delete one or more before creating a room.",
                MAX_NUM_ROOMS
            );
            vec![ServerMessage::error_from(&cause)]
        } else {
            writable_rooms.insert(room_name.clone(), Room::from(room_name));

            // must drop the writable lock for list_rooms to acquire a read lock
            drop(writable_rooms);

            // TODO: Just list the specific page that this room was inserted into
            self.list_all_rooms(None)
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

    pub fn list_all_rooms(&self, page_size: Option<usize>) -> Vec<ServerMessage> {
        let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE);

        let readable_rooms = match self.rooms.read() {
            Ok(lock) => lock,
            Err(_) => return vec![ServerMessage::error_from("Failed to get rooms lock")],
        };

        let total_pages = ((readable_rooms.len() as f64) / (page_size as f64)).ceil() as usize;

        let room_names = readable_rooms.keys().collect();

        let paginated_room_names = (0..total_pages)
            .map(|page_index| {
                ServerMessage::RoomList(RoomListPage::from(&room_names, page_index, page_size))
            })
            .collect();

        paginated_room_names
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
