use std::io::BufReader;
use std::io::BufWriter;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;

use bytes::{Buf, BufMut, BytesMut};

use crate::error::PeerError;

pub struct Handshake {
    pub pstrlen: [u8; 19],
    pub pstr: &'static str, // 写死的值, 所以直接用static
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

pub const PROTOCOL_STRING: &'static str = "BitTorrent protocol";

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            pstrlen: [0; 19],
            pstr: PROTOCOL_STRING,
            reserved: [0; 8],
            info_hash,
            peer_id,
        }
    }

    pub fn len(&self) -> u64 {
        19 + 8 + 20 + 20
    }

    // <pstrlen><pstr><reserved><info_hash><peer_id>
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        buf.put_u8(self.pstr.len() as u8);
        buf.extend_from_slice(&self.pstr.as_bytes());
        buf.extend_from_slice(&self.reserved);
        buf.extend_from_slice(&self.info_hash);
        buf.extend_from_slice(&self.peer_id);
        buf.to_vec()
    }

    pub fn from_bytes<T>(buf: T) -> Result<Handshake, PeerError>
    where
        T: AsRef<[u8]>,
    {
        let mut tmp_buf = Cursor::new(&buf);
        let pstrlen = tmp_buf.get_u8() as u8;
        // protocol string
        // let mut prot = [0; 19];
        // buf.copy_to_slice(&mut prot);
        // // reserved field
        // let mut reserved = [0; 8];
        // buf.copy_to_slice(&mut reserved);
        // // info hash
        // let mut info_hash = [0; 20];
        // buf.copy_to_slice(&mut info_hash);
        // // peer id
        // let mut peer_id = [0; 20];
        // buf.copy_to_slice(&mut peer_id);
        todo!()
    }
}
