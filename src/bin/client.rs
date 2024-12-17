use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::TcpStream;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
    user: Uuid,
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4000")?;

    println!("connected to {}", stream.peer_addr()?);

    let user = Message {
        message: String::from("helloooo"),
        user: Uuid::new_v4(),
    };
    let serialized_user = serde_json::to_string(&user).unwrap();

    stream.write_all(serialized_user.as_bytes())?;

    let buffer = [0; 128];
    let rcv = stream.read(&mut [0; 128])?;

    println!("received {}", String::from_utf8_lossy(&buffer[..rcv]));

    Ok(())
}
