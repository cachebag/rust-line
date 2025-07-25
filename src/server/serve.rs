// src/server/serve.rs

// use crate::error::RequestParseError;
use crate::http::{Parser, Response};
use std::io::Result;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn handle_request(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 512];
    let mut parser = Parser::new();
    let n = stream.read(&mut buffer)?;
    let request_str = String::from_utf8_lossy(&buffer[..n]);

    match parser.extract_and_validate_request(&request_str) {
        Ok((method, path, major, minor)) => {
            println!("{method:?} {path} HTTP/{major}.{minor}");
            let response = Response::ok("PING".to_string());
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
        Err(parse_error) => {
            eprintln!("{parse_error}");
            let response = "HTTP/1.1 400 Bad Request\r\n\r\nBad Request";
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}
