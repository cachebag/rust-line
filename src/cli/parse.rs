use crate::server::{Config, Mode};

pub fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Missing command".into());
    }

    match args[1].as_str() {
        "dir" | "directory" => {
            let dir = args.get(2).cloned().unwrap_or_else(|| ".".to_string());
            let port = args
                .get(3)
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(8080);
            Ok(Config {
                mode: Mode::Directory,
                port,
                directory: Some(dir),
            })
        }
        "ns" => {
            let port = args
                .get(2)
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(8080);
            Ok(Config {
                mode: Mode::NoDir,
                port,
                directory: None,
            })
        }
        "-h" | "help" => Err("help".into()),
        other => Err(format!("Unknown option: {other}")),
    }
}
