use rustline::server::runtime::run;

#[tokio::main]
async fn main() {
    run("127.0.0.1:8080".to_string()).await.unwrap(); 
}
