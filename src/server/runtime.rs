use crate::server::serve::Server;
use tokio::net::TcpListener;

pub async fn run(addr: String) -> std::io::Result<()> {
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
