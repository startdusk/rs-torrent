use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use crate::error::PeerError;

pub struct Handshake {
    pub pstrlen: [u8; 19],
    pub pstr: &'static str, // 写死的值, 所以直接用static
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            pstrlen: [0; 19],
            pstr: "BitTorrent protocol",
            reserved: [0; 8],
            info_hash,
            peer_id,
        }
    }

    pub fn len(&self) -> u64 {
        19 + 8 + 20 + 20
    }

    // <pstrlen><pstr><reserved><info_hash><peer_id>
    pub fn encode(&self) -> Result<Vec<u8>, PeerError> {
        let mut buf = Vec::with_capacity(self.pstr.len() + 49);
        buf.write_all(&self.pstrlen)?;
        buf.write_all(&self.pstr.as_bytes())?;
        buf.write_all(&self.reserved)?;
        buf.write_all(&self.info_hash)?;
        buf.write_all(&self.peer_id)?;
        Ok(buf)
    }

    pub fn from_bytes<T>(bytes: T) -> Result<Handshake, PeerError>
    where
        T: AsRef<[u8]>,
    {
        let mut buf = BufReader::new(bytes.as_ref());
        let mut len_buf: Vec<u8> = Vec::with_capacity(1);
        buf.read(&mut len_buf)?;

        todo!()
    }
}
