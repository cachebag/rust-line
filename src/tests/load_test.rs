use tokio::{
    net::TcpStream,
    io::{AsyncWriteExt, AsyncReadExt},
    task::JoinHandle,
};
use std::{
    error::Error,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

pub async fn run_load_test(connections: usize) -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let successes = Arc::new(AtomicUsize::new(0));
    let failures = Arc::new(AtomicUsize::new(0));
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(connections);

    for _ in 0..connections {
        let successes = Arc::clone(&successes);
        let failures = Arc::clone(&failures);

        let handle = tokio::spawn(async move {
            match TcpStream::connect(&addr).await {
                Ok(mut stream) => {
                    let result = async {
                        stream.write_all(b"GET /ping HTTP/1.1\r\nHost: localhost\r\n\r\n").await?;
                        let mut buf = [0; 64];
                        stream.read(&mut buf).await.map(|_| ())
                    }
                    .await;

                    match result {
                        Ok(_) => {
                            successes.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(e) => {
                            failures.fetch_add(1, Ordering::Relaxed);
                            eprintln!("[WARN] Connection succeeded but read/write failed: {e}");
                        }
                    }
                }
                Err(e) => {
                    failures.fetch_add(1, Ordering::Relaxed);
                    eprintln!("[WARN] Failed to connect: {e}");
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let ok = successes.load(Ordering::Relaxed);
    let err = failures.load(Ordering::Relaxed);

    println!(
        "[INFO] run_load_test: {connections} connections -> {ok} success, {err} failure"
    );

    Ok(())
}
