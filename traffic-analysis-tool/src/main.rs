use std::net::{IpAddr, Ipv4Addr};
use tokio::net::UdpSocket;

async fn trace_route(
    target: IpAddr,
    ttl: u8,
) -> Result<Option<IpAddr>, Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_ttl(ttl)?;

    let mut buf = [0; 16];
    let mut iter = (1..=4).cycle();
    let src_addr = IpAddr::V4(Ipv4Addr::new(
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ));
    socket.send_to(&[], &src_addr, &target).await?;

    let (bytes, src) = socket.recv_from(&mut buf).await?;
    if bytes >= 12 {
        Ok(Some(src.ip()))
    } else {
        Ok(None)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = "google.com";
    let target_addr = target.parse()?;
    for ttl in 1..=30 {
        match trace_route(target_addr, ttl).await? {
            Some(addr) => println!("{} ({})", addr, ttl),
            None => println!("*"),
        }
    }
    Ok(())
}
