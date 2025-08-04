use crate::server::serve::Server;
use tokio::net::TcpListener;
use std::env;
use std::error::Error;

const USAGE: &str = r#"
USAGE: cargo run <command> [args]

Options: 
        ns                 Run server with no directory specified
        directory <DIR>    File serving directory (default: .)
        -h, help           Show This help message
"#;

macro_rules! run_server {
    ($server:expr, $addr:expr, $label:expr) => {{
        let listener = TcpListener::bind(&$addr).await?;
        println!("Listening on http://{}\n{}", $addr, $label);
        loop {
            let (stream, _) = listener.accept().await?;
            let server = $server.clone();
            tokio::spawn(async move {
                if let Err(e) = server.handle_request(stream).await {
                    eprintln!("Error: {}", e);
                }
            });
        }
    }};
}

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
            let label = format!("In directory: {dir}");
            run_server!(server, addr, label);
        }

        "ns" => {
            let server = Server::new();
            let label = String::from("Serving with no specified directory");
            run_server!(server, addr, label);
        }

        "-h" | "help" => {
            println!("{USAGE}");
            Ok(())
        }

        _ => {
            eprintln!("Unknown option: {}\n{USAGE}", args[1]);
            std::process::exit(1);
        }
    }
}

