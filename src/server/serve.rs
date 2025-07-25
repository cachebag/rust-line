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

            let response = match path.as_str() {
                "/" => Response::ok("Hello, World!\n".to_string()),
                "/ping" => Response::ok("pong\n".to_string()),
                "/health" => Response::ok("Server is healthy\n".to_string()),
                _ => Response::not_found(),
            };

            stream.write_all(response.to_string().as_bytes())?;
            stream.flush()?;
        }
        Err(parse_error) => {
            eprintln!("{parse_error}");
            let response = Response::bad_request();
            stream.write_all(response.to_string().as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}
