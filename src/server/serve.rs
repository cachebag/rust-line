// src/server/serve.rs

// use crate::error::RequestParseError;
use crate::http::{Parser, Response};
use std::io::Result;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Clone)]
pub struct Server {
    pub start_time: Instant,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    fn handle_uptime(&self) -> Response {
        let uptime = self.start_time.elapsed();
        Response::ok(format!("Server Uptime {:?}\n", uptime))
    }

    pub async fn handle_request(&self, mut stream: TcpStream) -> Result<()> {
        let mut buffer = [0; 512];
        let mut parser = Parser::new();
        let n = stream.read(&mut buffer).await?;
        let request_str = String::from_utf8_lossy(&buffer[..n]);

        match parser.extract_and_validate_request(&request_str) {
            Ok((method, path, major, minor, headers)) => {
                println!("{method:?} {path} HTTP/{major}.{minor}");
                println!("\n");
                let response = match path.strip_prefix('/') {
                    Some(rest) => {
                        if let Some(arg) = rest.strip_prefix("echo/") {
                            if arg.is_empty() {
                                Response::bad_request()
                            } else {
                                let mut text = String::from(arg);
                                text.push('\n');
                                Response::ok(text)
                            }
                        } else {
                            match rest {
                                "ping" => Response::ok("PONG\n".to_string()),
                                "uptime" => self.handle_uptime(),
                                "echo" => Response::bad_request(),
                                "user-agent" => {
                                    let ua = headers
                                        .get("User-Agent")
                                        .map(|s| s.as_str())
                                        .unwrap_or("(none)");

                                    Response::ok(format!("{}\n", ua))
                                }
                                "headers" => {
                                    let mut body = String::new();
                                    for (k, v) in &headers {
                                        body.push_str(&format!("{}: {}\n", k, v));
                                    }
                                    Response::ok(body)
                                }
                                _ => Response::not_found(),
                            }
                        }
                    }
                    _ => Response::not_found(),
                };

                stream.write_all(response.to_string().as_bytes()).await?;
                stream.flush().await?;
            }
            Err(parse_error) => {
                eprintln!("{parse_error}");
                let response = Response::bad_request();
                stream.write_all(response.to_string().as_bytes()).await?;
                stream.flush().await?;
            }
        }

        Ok(())
    }
}
