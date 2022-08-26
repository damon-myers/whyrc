use whyrc_protocol::ClientMessage;

fn main() {
    let ping = ClientMessage::Ping;

    let serialized_ping = serde_json::to_string(&ping).unwrap();
    println!("Serialized ping:\n{}", serialized_ping);

    println!("Number of bytes: {}", serialized_ping.as_bytes().len());
}
