use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:33434").await?;

    let mut port = 33434;
    let remote_addr = format!("google.com:{}", port);
    sock.set_ttl(1)?;
    sock.connect(remote_addr).await?;
    let mut buf = [0; 1024];
    println!("{}", sock.send(&buf).await?);
    sock.recv(&mut buf).await?;

    Ok(())
}
