use std::io::Write;

use crate::error::PeerError;
use shared::bytes::ByteBuffer;
use tokio::runtime::Handle;

#[derive(Default)]
pub struct Handshake {
    pub pstrlen: u8,
    pub pstr: &'static str, // 写死的值, 所以直接用static
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

pub const PROTOCOL_STRING: &'static str = "BitTorrent protocol";

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            pstrlen: PROTOCOL_STRING.len() as u8,
            pstr: PROTOCOL_STRING,
            reserved: [0; 8],
            info_hash,
            peer_id,
        }
    }

    pub fn len() -> usize {
        1 + 19 + 8 + 20 + 20
    }

    // <pstrlen><pstr><reserved><info_hash><peer_id>
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::len());
        buf.extend_from_slice(&[self.pstrlen]);
        buf.extend_from_slice(&self.pstr.as_bytes());
        buf.extend_from_slice(&self.reserved);
        buf.extend_from_slice(&self.info_hash);
        buf.extend_from_slice(&self.peer_id);
        buf
    }

    pub fn decode<T>(bytes: T) -> Result<Handshake, PeerError>
    where
        T: AsRef<[u8]>,
    {
        let content_len = bytes.as_ref().len();
        if content_len != Self::len() {
            // TODO: invalid content
        }
        let mut buf = ByteBuffer::new(bytes.as_ref());
        let mut obj = Handshake::default();
        if let Some(&pstrlen) = buf.next() {
            obj.pstrlen = pstrlen
        }

        // let pstr: Vec<u8> = buf.take(PROTOCOL_STRING.len()).cloned().collect();
        // let reserved: Vec<u8> = buf.take(8).cloned().collect();
        // let info_hash: Vec<u8> = buf.take(20).cloned().collect();
        // let peer_id: Vec<u8> = buf.take(20).cloned().collect();
        // Ok(Self {
        //     pstrlen: todo!(),
        //     pstr: todo!(),
        //     reserved: todo!(),
        //     info_hash: todo!(),
        //     peer_id: todo!(),
        // })

        todo!()
    }
}
