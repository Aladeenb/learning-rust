use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() {
    let timeout = Duration::from_secs(1);
    let addr = "127.0.0.1:0".to_socket_addrs().unwrap().next().unwrap();
    // TODO let the user decides which address to scan (in case of admin)
    // TODO is it possible to scan sockets from different ips?
    // TODO let the user decides which ports to scan?

    // TODO add loading message
    for port in 1..65535 {
        match TcpStream::connect_timeout(&addr, timeout) {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => continue, // We are not interested in printing the closed ports. hence `continue` skips to the next iter.
        }
    }
    print!("Scan completed!")
    // TODO reformat ouput
}
