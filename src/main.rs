use std::io::prelude::*;
use std::{
    io::Result,
    net::{TcpListener, TcpStream},
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server running on http:://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    println!("Got: {}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}
