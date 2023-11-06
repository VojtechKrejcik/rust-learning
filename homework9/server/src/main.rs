use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use serde_cbor;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>),  // Filename and its content as bytes
}

fn deserialize_message(data: &[u8]) -> Result<MessageType, serde_cbor::Error> {
    return serde_cbor::from_slice(data);
}

fn handle_client(mut stream: TcpStream) {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).unwrap();
    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    if let Err(e) = stream.read_exact(&mut buffer) {
        eprintln!("Failed to read message: {}", e);
        return;
    }

    match deserialize_message(&buffer) {
        Ok(message) => {
            // Handle the message
            println!("Received a message: {:?}", message);
            // Depending on the MessageType, do different things here
        }
        Err(e) => {
            // Handle the error, maybe log it or send an error message back
            eprintln!("Failed to deserialize message: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("localhost:1111").unwrap();
    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.insert(addr.clone(), stream);

        handle_client(clients.get(&addr).unwrap().try_clone().unwrap());
    }
}