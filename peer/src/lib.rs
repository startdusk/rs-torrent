use std::net::SocketAddr;

use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct Handshake {
    
}

pub struct Peer;

impl Peer {
    pub async fn handshake(addr: SocketAddr) {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        let mut buf = Vec::new();
        stream.read_buf(&mut buf).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_tcp_conn() {
        let cases = [SocketAddr::new("198.54.132.42".parse().unwrap(), 54886)];
        for addr in cases {
            Peer::handshake(addr).await;
        }
    }
}
