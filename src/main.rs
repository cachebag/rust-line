use rustline::server::runtime::run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
