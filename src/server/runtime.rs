use crate::server::serve::Server;
use tokio::net::TcpListener;
use std::env;
use std::error::Error;

const USAGE: &str = r#"
USAGE: cargo run <command> [args]

Options: 
        directory <DIR>    File serving directory (default: .)
        -h, help           Show This help message
"#;

pub async fn run(addr: String) -> Result<(), Box<dyn Error>> {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "directory" => {
            let dir = if args.len() >= 3 {
                args[2].clone()
            } else {
                ".".to_string()
            };
            let server = Server::new_with_directory(dir.clone());
            let addr = addr.to_string();
            let listener = TcpListener::bind(&addr).await?;
            println!("Listening on http://127.0.0.1:8080\n In directory: {dir}");

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
        "-h" | "help" => {
            println!("{USAGE}");
            return Ok(())
        }
        _ => eprintln!("Unknown option: {}\n{USAGE}", args[1])
    }

    let addr = addr.to_string();
    let listener = TcpListener::bind(&addr).await?;
    let server = Server::new();
    println!("Listening on http://127.0.0.1:8080\n");

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
