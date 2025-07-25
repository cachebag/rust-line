use rustline::server::serve::handle_request;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_request(stream)?;
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }
    Ok(())
}
