use tokio::net::TcpStream;

async fn scan_port(host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", host, port);
    match TcpStream::connect(&addr).await {
        Ok(_) => println!("Port {} is open", port),
        Err(_) => println!("Port {} is closed", port),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "localhost";
    for port in 1..1024 {
        tokio::spawn(async move {
            scan_port(host, port).await.unwrap();
        });
    }
    Ok(())
    // TODO reformat output
}
