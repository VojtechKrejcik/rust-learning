
use std::net::TcpStream;
use std::io::Write;
use std::io::{stdin, ErrorKind, Read};
use std::env;
use serde::{Serialize, Deserialize};
use std::path::Path;
use serde_cbor;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>)
}

fn serialize_message(message: &MessageType) -> Vec<u8> {
   return serde_cbor::to_vec(&message).unwrap();
}

fn send_message(address: &str, message: &MessageType) {
    let serialized = serialize_message(message);
    let mut stream = TcpStream::connect(address).unwrap();

    // Send the length of the serialized message (as 4-byte value).
    let len = serialized.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();

    // Send the serialized message.
    stream.write_all(&serialized).unwrap();
}

fn file_to_bytes(path:&Path) -> std::io::Result<Vec<u8>> {
    let  mut file = File::open(path)?;
    let mut file_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_data)?;
    Ok(file_data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let address:String=  if args.len() == 1 {
        String::from("localhost:1111")
    }else if args.len() == 3{
        format!("{}:{}", args[1], args[2])
    }else{
        String::from("")
    };
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input)?; 
        
        let args: Vec<&str> = input.split_whitespace().collect();
        
        
        match args[0] {
            ".file" => {
                let path = Path::new(args[1]);
                match file_to_bytes(&path) {
                    Ok(file_data) => {
                        let msg = MessageType::File(String::from(args[1]), file_data);
                        send_message(&address, &msg);
                    }
                    Err(e) => {
                        println!("Error reading file: {}", e);
                        continue;
                    }
                }

            }
            ".image" => send_message(&address, &MessageType::Image(input.as_bytes().to_vec())),
            ".quit" => break(Ok(())),
            _ => send_message(&address, &MessageType::Text(String::from(input)))
        }
    }
}
