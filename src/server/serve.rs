// src/server/serve.rs 
use std::io::prelude::*;
use std::io::Result;
use std::net::TcpStream;
use crate::http;

pub fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    let request_str = String::from_utf8_lossy(&buffer[..n]);

    println!("{request_str}");
    Ok(())
}
