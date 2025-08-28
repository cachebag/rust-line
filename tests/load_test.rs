use rustline::server::runtime::run;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

// TODO: Refactor to pass a port into a test
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn integration_load_test() {
    tokio::spawn(async {
        run("127.0.0.1:8080".to_string()).await.unwrap();
    });
    tokio::time::sleep(Duration::from_secs(1)).await;

    let addr = "127.0.0.1:8080";
    let total_connections = 10_000;

    let mut handles = Vec::with_capacity(total_connections);

    for _ in 0..total_connections {
        let addr = addr.to_string();
        let handle = tokio::spawn(async move {
            match TcpStream::connect(&addr).await {
                Ok(mut stream) => {
                    let _ = stream
                        .write_all(b"GET /ping HTTP/1.1\r\nHost: localhost\r\n\r\n")
                        .await;
                    let mut buf = [0; 64];
                    let _ = stream.read(&mut buf).await;
                }
                Err(e) => {
                    panic!("Failed to connect to server: {e}");
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
