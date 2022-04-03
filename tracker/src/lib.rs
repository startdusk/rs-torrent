use std::net::IpAddr;

pub struct Peer {
    pub peer_id: String,
    pub ip: IpAddr,
    pub port: usize,
}

pub enum Peers {
    List(Vec<Peer>),
    String(String),
}

pub enum Event {
    Started,
    Completed,
    Stopped,
    None,
}

impl Event {
    pub fn event(&self) -> Option<String> {
        match *self {
            Self::Started => Some("started".to_owned()),
            Self::Completed => Some("completed".to_owned()),
            Self::Stopped => Some("stopped".to_owned()),
            Self::None => None,
        }
    }
}

pub struct Request {
    pub info_hash: String,
    pub peer_id: String,
    pub port: usize,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: usize,
    pub compact: usize,
    pub no_peer_id: Option<bool>,
    pub event: Event,
    pub ip: Option<IpAddr>,
    pub nuwant: usize,
    pub key: Option<String>,
    pub tracker_id: Option<String>,
}

pub struct Response {
    pub failure_reason: String,
    pub warning_message: Option<String>,
    pub interval: usize,
    pub min_interval: Option<usize>,
    pub tracker_id: String,
    pub complete: usize,
    pub incomplete: usize,
    pub peers: Peers,
}
