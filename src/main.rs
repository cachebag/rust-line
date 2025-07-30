use rustline::server::serve::Server;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let hostname = "127.0.0.1";
    let port = 8080;
    let addr = format!("{}:{}", hostname, port);
    let listener = TcpListener::bind(&addr)?;
    let server = Server::new();
    println!("Listening on http://127.0.0.1:8080\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                server.handle_request(stream)?;
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }
    Ok(())
}
