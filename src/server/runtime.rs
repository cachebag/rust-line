use crate::server::serve::Server;
use std::error::Error;
use tokio::net::TcpListener;

pub const USAGE: &str = r#"
USAGE: cargo run <command> [args]

Commands:
    ns [PORT]            Run server with no directory specified
    dir <DIR> [PORT]     File serving directory (default: .)
    -h, help             Show this help message
"#;

#[derive(Clone, Debug)]
pub enum Mode {
    NoDir,
    Directory,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub mode: Mode,
    pub port: u16,
    pub directory: Option<String>,
}

/// Launch the HTTP server with the given configuration.
///
/// This function does **not** parse CLI arguments or exit the process,
/// so it’s safe to call from integration tests.
pub async fn run_with(cfg: Config) -> Result<(), Box<dyn Error>> {
    let addr = format!("127.0.0.1:{}", cfg.port);
    let listener = TcpListener::bind(&addr).await?;

    let server = match cfg.mode {
        Mode::Directory => {
            let dir = cfg.directory.clone().unwrap_or_else(|| ".".into());
            Server::new_with_directory(dir)
        }
        Mode::NoDir => Server::new(),
    };

    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.handle_request(stream).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}
