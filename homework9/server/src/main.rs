use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_cbor;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>), // Filename and its content as bytes
}

fn deserialize_message(data: &[u8]) -> Result<MessageType, serde_cbor::Error> {
    return serde_cbor::from_slice(data);
}

fn handle_client(mut stream: TcpStream) {
    let mut len_bytes = [0u8; 4];
    if let Err(e) = stream.read_exact(&mut len_bytes) {
        eprintln!("Failed to read length: {}", e);
        return;
    }
    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    if let Err(e) = stream.read_exact(&mut buffer) {
        eprintln!("Failed to read message: {}", e);
        return;
    }

    match deserialize_message(&buffer) {
        Ok(MessageType::File(filename, file_data)) => {
            println!("Received a file: {:?}", filename);
            if let Err(e) = save_to_file(&("files/".to_owned() + &filename), file_data) {
                eprintln!("Failed to save file: {}", e);
            }
        }
        Ok(MessageType::Image(file_data)) => {
            println!("Received a image...");
            let filename = Utc::now().format("%Y%m%d%H%M%S").to_string();
            if let Err(e) = save_to_file(&("images/".to_owned() + &filename), file_data) {
                eprintln!("Failed to save file: {}", e);
            }
        }
        Ok(MessageType::Text(text)) => println!("{}", text),
        Err(e) => {
            eprintln!("Failed to deserialize message: {}", e);
        }
    }
}

fn save_to_file(file_path: &str, bytes: Vec<u8>) -> io::Result<()> {
    fs::write(file_path, bytes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(Path::new("images"))?;
    fs::create_dir_all(Path::new("files"))?;
    let listener = TcpListener::bind("localhost:1111").unwrap();
    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.insert(addr.clone(), stream);

        handle_client(clients.get(&addr).unwrap().try_clone().unwrap());
    }
    Ok(())
}
