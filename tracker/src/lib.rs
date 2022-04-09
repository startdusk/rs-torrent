use std::{io, net::IpAddr};

use bencode::BenObject;
use error::TrackerError;
use torrent::Sha1Hash;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};
use reqwest::{Client, Url};

pub mod error;

/// Contains the characters that need to be URL encoded according to:
/// https://en.wikipedia.org/wiki/Percent-encoding#Types_of_URI_characters
const URL_ENCODE_RESERVED: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'~')
    .remove(b'.');

pub type PeerId = [u8; 20];

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
}

impl Event {
    pub fn event(&self) -> String {
        match *self {
            Self::Started => "started".to_string(),
            Self::Completed => "completed".to_string(),
            Self::Stopped => "stopped".to_string(),
        }
    }
}

pub struct Request {
    pub info_hash: Sha1Hash,
    pub peer_id: PeerId,
    pub port: usize,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: usize,
    pub compact: usize,
    pub no_peer_id: Option<bool>,
    pub event: Option<Event>,
    pub ip: Option<IpAddr>,
    pub numwant: Option<usize>,
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

pub struct Tracker {
    client: Client,
    url: Url,
}

impl Tracker {
    pub fn new(url: Url) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    pub async fn find_peers(&self, req: Request) -> Result<Response, TrackerError> {
        let query = self.build_query(&req);
        let url = self.build_url(&req);
        // send request
        let resp = self
            .client
            .get(&url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        self.parse_bytes(resp.to_vec())
    }

    fn parse_bytes<T>(&self, bytes: T) -> Result<Response, TrackerError>
    where
        T: AsRef<[u8]>,
    {
        let mut obj = BenObject::from_bytes(bytes)?;
        match obj {
            BenObject::Dict(ref mut dict) => Ok(Response {
                failure_reason: todo!(),
                warning_message: todo!(),
                interval: todo!(),
                min_interval: todo!(),
                tracker_id: todo!(),
                complete: todo!(),
                incomplete: todo!(),
                peers: todo!(),
                // info: Self::info(dict)?,
                // announce: Self::announce(dict)?,
                // announce_list: Self::announce_list(dict)?,
                // creation_date: Self::creation_date(dict)?,
                // comment: Self::comment(dict)?,
                // created_by: Self::created_by(dict)?,
                // encoding: Self::encoding(dict)?,
            }),
            _ => return Err(TrackerError::InvalidResponse),
        }
    }

    fn build_url(&self, req: &Request) -> String {
        format!(
            "{url}\
            ?info_hash={info_hash}\
            &peer_id={peer_id}",
            url = self.url,
            info_hash = percent_encoding::percent_encode(&req.info_hash, URL_ENCODE_RESERVED),
            peer_id = percent_encoding::percent_encode(&req.peer_id, URL_ENCODE_RESERVED),
        )
    }

    fn build_query(&self, req: &Request) -> Vec<(&str, String)> {
        let mut query = vec![
            ("port", req.port.to_string()),
            ("uploaded", req.uploaded.to_string()),
            ("downloaded", req.downloaded.to_string()),
            ("left", req.left.to_string()),
            ("compact", req.compact.to_string()),
        ];
        if req.compact != 1 {
            query.push(("no_peer_id", "".to_string()));
        }
        if let Some(event) = &req.event {
            query.push(("event", event.event()));
        }
        if let Some(ip) = req.ip {
            query.push(("ip", ip.to_string()))
        }
        if let Some(numwant) = req.numwant {
            query.push(("numwant", numwant.to_string()))
        }
        if let Some(key) = &req.key {
            query.push(("key", key.to_string()))
        }
        if let Some(tracker_id) = &req.tracker_id {
            query.push(("trackerid", tracker_id.to_string()))
        }
        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_tracker_url() {
        let tracker = Tracker::new("http://bttracker.debian.org:6969/announce".parse().unwrap());
        tracker
            .find_peers(Request {
                info_hash: [
                    40, 197, 81, 150, 245, 119, 83, 196, 10, 206, 182, 251, 88, 97, 126, 105, 149,
                    167, 237, 219,
                ],
                peer_id: [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ],
                port: 6881,
                uploaded: 0,
                downloaded: 0,
                left: 39631728,
                compact: 1,
                no_peer_id: None,
                event: None,
                numwant: None,
                key: None,
                tracker_id: None,
                ip: None,
            })
            .await
            .unwrap();
    }
}

// 构建tracker url
// 请求url获取响应
// 解析响应
