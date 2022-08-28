use whyrc_protocol::ClientMessage;

fn main() {
    let ping = ClientMessage::Ping;

    let serialized_ping = serde_json::to_string(&ping).unwrap();
    println!("Serialized ping:\n{}", serialized_ping);
    println!("Number of bytes: {}", serialized_ping.as_bytes().len());

    let create_room = ClientMessage::CreateRoom {
        name: String::from("test"),
    };
    let serialized_create_room = serde_json::to_string(&create_room).unwrap();
    println!("Serialized create_room:\n{}", serialized_create_room);
    println!(
        "Number of bytes: {}",
        serialized_create_room.as_bytes().len()
    );

    let list_rooms = ClientMessage::ListRooms {
        page: 0,
        page_size: 100,
    };
    let serialized_list_rooms = serde_json::to_string(&list_rooms).unwrap();
    println!("Serialized list_rooms:\n{}", serialized_list_rooms);
    println!(
        "Number of bytes: {}",
        serialized_list_rooms.as_bytes().len()
    );
}
