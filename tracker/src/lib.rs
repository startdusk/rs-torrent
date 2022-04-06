use std::net::IpAddr;

use url::{ParseError, Url};

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

pub struct Tracker;

impl Tracker {
    pub fn build_url(announce: String, req: Request) -> Result<String, ParseError> {
        let url = Url::parse_with_params(
            announce.as_str(),
            &[
                ("info_hash", req.info_hash),
                ("peer_id", req.peer_id),
                ("port", req.port.to_string()),
                ("uploaded", req.uploaded.to_string()),
                ("downloaded", req.downloaded.to_string()),
                ("compact", req.compact.to_string()),
                ("left", req.left.to_string()),
            ],
        )?;
        Ok(url.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tracker_url() {
        let url = Tracker::build_url(
            "http://bttracker.debian.org:6969/announce?".to_string(),
            Request {
                info_hash: "xxxxxxxxxxx".to_string(),
                peer_id: "127.0.0.1".to_string(),
                port: 56789,
                uploaded: 0,
                downloaded: 0,
                left: 1,
                compact: 0,
                no_peer_id: None,
                event: Event::None,
                ip: None,
                nuwant: 0,
                key: None,
                tracker_id: None,
            },
        )
        .unwrap();
        assert_eq!(url, "http://bttracker.debian.org:6969/announce?info_hash=xxxxxxxxxxx&peer_id=127.0.0.1&port=56789&uploaded=0&downloaded=0&compact=0&left=1".to_string());
    }
}

// 构建tracker url
// 请求url获取响应
// 解析响应
