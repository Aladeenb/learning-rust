use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

// wrap our func into tokio main
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    loop {
        let mut buffer = [0u8; 1024];

        let bytes_read = socket.read(&mut buffer).await.unwrap();

        socket.write_all(&buffer[..bytes_read]).await.unwrap(); // .. = up to
    }

    /*
        for now, only one client can read and writes. Also only one message is supported.
        The user have to close the connection and run again to send a new message.
    */
    // TODO make the program supports multiple messages
}
