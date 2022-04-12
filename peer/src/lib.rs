mod error;
mod handshake;

use std::{io::copy, net::SocketAddr};

use tokio::{io::AsyncReadExt, net::TcpStream};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MessageValue {
    Choke = 0,
    Unchoke = 1,
    Interested = 2,
    NotInterested = 3,
    Have = 4,
    Bitfield = 5,
    Request = 6,
    Block = 7,
    Cancel = 8,
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
