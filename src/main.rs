use rustline::cli::parse_args;
use rustline::server::runtime::{USAGE, run_with};

#[tokio::main]
async fn main() {
    match parse_args() {
        Ok(cfg) => {
            if let Err(e) = run_with(cfg).await {
                eprintln!("Server error: {e}");
                std::process::exit(1);
            }
        }
        Err(ref e) if e == "help" => {
            println!("{USAGE}");
        }
        Err(e) => {
            eprintln!("{e}\n{USAGE}");
            std::process::exit(1);
        }
    }
}
