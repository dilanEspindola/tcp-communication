use serde::Deserialize;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
use uuid::Uuid;

#[derive(Deserialize)]
struct Message {
    message: String,
    user: Uuid,
}

fn handle_client_connection(mut stream: TcpStream) {
    let mut buffer = [0; 128];
    let buffer_read = stream.read(&mut buffer).unwrap();

    let deserialized: Message = serde_json::from_slice(&buffer[..buffer_read]).unwrap();

    println!("{}: {}\n", deserialized.user, deserialized.message);

    stream.write_all(b"message").unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    println!("server listening on port 4000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client_connection(stream),

            Err(e) => {
                eprintln!("failed to accept: {}", e);
            }
        }
    }

    Ok(())
}
