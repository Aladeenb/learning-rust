use std::net::TcpListener;

// wrap our func into tokio main
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await;
}
