// src/server/serve.rs
use crate::http;
use crate::error::RequestParseError;
use std::io::Result;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    let request_str = String::from_utf8_lossy(&buffer[..n]);

    match http::Parser::extract_and_validate_request(&request_str) {
        Ok((method, path, major, minor)) => {
            println!("Parsed: {method:?} {path} HTTP/{major}.{minor}"); 
            // TODO: Successful requests 
        }
        Err(parse_error) => {
            eprintln!("Parse error: {parse_error}");
            // TODO: Error responses
        }
    }

    Ok(())
}
