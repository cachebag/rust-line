use std::error::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    task::JoinHandle,
};

pub async fn run_load_test(connections: usize) -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let mut handles: Vec<JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>> =
        Vec::with_capacity(connections);

    for _ in 0..connections {
        let addr = addr.to_string();
        let handle = tokio::spawn(async move {
            match TcpStream::connect(&addr).await {
                Ok(mut stream) => {
                    let _ = stream
                        .write_all(b"GET /ping HTTP/1.1\r\nHost: localhost\r\n\r\n")
                        .await;
                    let mut buf = [0; 64];
                    let _ = stream.read(&mut buf).await;
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(Ok(())) => {}                   // success
            Ok(Err(e)) => return Err(e),       // inner error
            Err(e) => return Err(Box::new(e)), // task join error
        }
    }

    Ok(())
}
