use crate::server::serve::Server;
use std::env;
use std::error::Error;
use tokio::net::TcpListener;

const USAGE: &str = r#"
USAGE: cargo run <command> [args]

Commands: 
        ns [PORT]                Run server with no directory specified
        dir <DIR> [PORT]         File serving directory (default: .)
        -h, help                 Show This help message
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

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    }

    let mut port = "8080".to_string(); // default port 
    let mut used_default_port = true;

    match args[1].as_str() {
        "dir" | "directory" => {
            if args.len() >= 4 {
                port = args[3].clone();
                used_default_port = false;
            }
            let dir = if args.len() >= 3 {
                args[2].clone()
            } else {
                ".".to_string()
            };
            let addr = format!("127.0.0.1:{port}");
            let server = Server::new_with_directory(dir.clone());
            let label = if used_default_port {
                format!("In directory: {dir} (default port: {port})")
            } else {
                format!("In directory: {dir}")
            };
            run_server!(server, addr, label);
        }

        "ns" => {
            if args.len() >= 3 {
                port = args[2].clone();
                used_default_port = false;
            }
            let addr = format!("127.0.0.1:{port}");
            let server = Server::new();
            let label = if used_default_port {
                String::from("Serving with no specified directory (default port 8080)")
            } else {
                String::from("Serving with no specified directory")
            };
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
