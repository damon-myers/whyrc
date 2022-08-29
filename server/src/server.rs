use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use whyrc_protocol::{ClientMessage, RoomList, ServerMessage};

use crate::room::Room;

type RoomMap = BTreeMap<String, Room>;

#[derive(Clone)]
pub struct Server {
    ip_address: String,
    port: u16,
    password: String,
    rooms: Arc<RwLock<RoomMap>>,
}

impl Server {
    pub fn from(args: crate::Args) -> Self {
        Server {
            ip_address: args.ip_address,
            port: args.port,
            password: args.password.unwrap(),
            rooms: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn handle_message(&self, message: ClientMessage) -> ServerMessage {
        match message {
            ClientMessage::Ping => ServerMessage::Pong,
            ClientMessage::CreateRoom { name } => self.add_room(name),
            ClientMessage::DeleteRoom { name } => self.remove_room(name),
            ClientMessage::ListRooms { page, page_size } => self.list_rooms(page, page_size),
        }
    }

    fn add_room(&self, room_name: String) -> ServerMessage {
        let writable_rooms = self.rooms.write();

        if writable_rooms.is_err() {
            return ServerMessage::error_from("Failed to obtain rooms lock");
        }

        let mut writable_rooms = writable_rooms.unwrap();
        if writable_rooms.contains_key(&room_name) {
            let cause = format!("Room with name {} already exists", room_name);
            ServerMessage::error_from(&cause)
        } else {
            writable_rooms.insert(room_name.clone(), Room::from(room_name));
            ServerMessage::Ack
        }
    }

    fn remove_room(&self, room_name: String) -> ServerMessage {
        let writable_rooms = self.rooms.write();

        if writable_rooms.is_err() {
            return ServerMessage::error_from("Failed to obtain rooms lock");
        }

        let mut writable_rooms = writable_rooms.unwrap();
        if !writable_rooms.contains_key(&room_name) {
            let cause = format!("Room with name {} doesn't exist", room_name);
            ServerMessage::error_from(&cause)
        } else {
            writable_rooms.remove(&room_name);
            ServerMessage::Ack
        }
    }

    fn list_rooms(&self, page: usize, page_size: usize) -> ServerMessage {
        let readable_rooms = self.rooms.read();

        if let Ok(readable_rooms) = readable_rooms {
            ServerMessage::RoomList(RoomList::from(
                readable_rooms.keys().collect(),
                page,
                page_size,
            ))
        } else {
            ServerMessage::error_from("Failed to obtain rooms lock")
        }
    }
}
