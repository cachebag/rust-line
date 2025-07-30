use rustline::server::serve::Server;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let hostname = "127.0.0.1";
    let port = 8080;
    let addr = format!("{}:{}", hostname, port);
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
