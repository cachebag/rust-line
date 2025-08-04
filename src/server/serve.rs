// src/server/serve.rs

// use crate::error::RequestParseError;
use crate::http::{Parser, Response};
use std::io::Result;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::path::Path;

#[derive(Clone)]
pub struct Server {
    pub start_time: Instant,
    directory: Option<String>,
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
            directory: None,
        }
    }

    pub fn new_with_directory(dir: String) -> Self {
        Self {
            start_time: Instant::now(),
            directory: Some(dir),
        }
    }

    fn handle_uptime(&self) -> Response {
        let uptime = self.start_time.elapsed();
        Response::ok(format!("Server Uptime {:?}\n", uptime))
    }

    pub async fn handle_request(&self, mut stream: TcpStream) -> Result<()> {
        let mut buffer = [0; 8192];
        let mut parser = Parser::new();

        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

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
                                    path if path.starts_with("files/") => {
                                        let file_path = path.strip_prefix("files/").unwrap_or("");
                                        self.handle_file_request(file_path).await
                                    }
                                    _ => Response::not_found(),
                                }
                            }
                        }
                        _ => Response::not_found(),
                    };

                    stream.write_all(response.to_string().as_bytes()).await?;
                    stream.flush().await?;

                    if headers
                        .get("Connection")
                        .map(|v| v.eq_ignore_ascii_case("close"))
                        .unwrap_or(false)
                    {
                        break;
                    }
                }
                Err(_) => {
                    let response = Response::bad_request();
                    stream.write_all(response.to_string().as_bytes()).await?;
                    stream.flush().await?;
                    break;
                }
            }

            parser = Parser::new();
        }

        Ok(())
    }

    pub async fn handle_file_request(&self, file_path: &str) -> Response {
        let base_dir = self.directory.as_deref().unwrap_or(".");
        let full_path = Path::new(base_dir).join(file_path);

        if file_path.contains("..") {
            return Response::bad_request();
        }

        match tokio::fs::read_to_string(&full_path).await {
            Ok(contents) => {
                // println!("File contents: {contents}")
                Response::ok(contents)
                    .content_type("application/octet-stream")
            }
            Err(e) => {
                eprintln!("Failed to read file: {e}");
                Response::not_found()
            }
        }
    
    }
}
