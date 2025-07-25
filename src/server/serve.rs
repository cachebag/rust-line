// src/server/serve.rs
// use crate::error::RequestParseError;
use crate::http::Parser;
use std::io::Result;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 512];
    let mut parser = Parser::new();
    let n = stream.read(&mut buffer)?;
    let request_str = String::from_utf8_lossy(&buffer[..n]);

    match parser.extract_and_validate_request(&request_str) {
        Ok((method, path, major, minor)) => {
            println!("Parsed: {method:?} {path} HTTP/{major}.{minor}");
            // TODO: Successful requests
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
            stream.write_all(response.as_bytes())?;
        }
        Err(parse_error) => {
            eprintln!("Parse error: {parse_error}");
            // TODO: Error responsee
            let response = "HTTP/1.1 400 Bad Request\r\n\r\nBad Request";
            stream.write_all(response.as_bytes())?;
        }
    }

    Ok(())
}
