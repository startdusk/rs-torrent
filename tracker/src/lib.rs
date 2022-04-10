use bytes::Buf;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use std::{borrow::Cow, net::SocketAddr};

use bencode::{BenObject, Dict};
use error::TrackerError;
use torrent::Sha1Hash;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};
use reqwest::{Client, Url};

pub mod error;

// encode url的时候，保留一些字符 info_hash 和 peer_id encode需要保留下面的字符
// https://en.wikipedia.org/wiki/Percent-encoding#Types_of_URI_characters
/// Note that all binary data in the URL (particularly info_hash and peer_id) must be properly escaped.
/// This means any byte not in the set 0-9, a-z, A-Z, '.', '-', '_' and '~',
/// must be encoded using the "%nn" format, where nn is the hexadecimal value of the byte. (See RFC1738 for details.)
/// For a 20-byte hash of \x12\x34\x56\x78\x9a\xbc\xde\xf1\x23\x45\x67\x89\xab\xcd\xef\x12\x34\x56\x78\x9a,
/// The right encoded form is %124Vx%9A%BC%DE%F1%23Eg%89%AB%CD%EF%124Vx%9A
const URL_ENCODE_RESERVED: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'~')
    .remove(b'.');

pub type PeerId = [u8; 20];

#[derive(Debug)]
pub struct Peer {
    pub peer_id: String,
    pub ip: IpAddr,
    pub port: usize,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Request {
    pub info_hash: Sha1Hash,
    pub peer_id: PeerId,
    pub port: usize,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: i64,
    pub compact: usize,
    pub no_peer_id: Option<bool>,
    pub event: Option<Event>,
    pub ip: Option<IpAddr>,
    pub numwant: Option<usize>,
    pub key: Option<String>,
    pub tracker_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub failure_reason: Option<String>,
    pub warning_message: Option<String>,
    pub interval: Duration,
    pub min_interval: Option<Duration>,
    pub tracker_id: Option<String>,
    pub complete: Option<i64>,
    pub incomplete: Option<i64>,
    pub peers: Vec<SocketAddr>,
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
                failure_reason: self.failure_reason(dict)?,
                warning_message: self.warning_message(dict)?,
                interval: self.interval(dict)?,
                min_interval: self.min_interval(dict)?,
                tracker_id: self.tracker_id(dict)?,
                complete: self.complete(dict)?,
                incomplete: self.incomplete(dict)?,
                peers: self.peers(dict)?,
            }),
            _ => return Err(TrackerError::InvalidResponse),
        }
    }
    fn port(&self, dict: &mut Dict) -> Result<u16, TrackerError> {
        match dict.remove("port") {
            Some(BenObject::Int(port)) => Ok(port as u16),
            Some(_) => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`port` does not map to int.",
                )))
            }
            None => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`port` does not exist.",
                )))
            }
        }
    }
    fn ip(&self, dict: &mut Dict) -> Result<IpAddr, TrackerError> {
        match dict.remove("ip") {
            Some(BenObject::String(ip)) => {
                let ip = if let Ok(ip) = ip.parse() {
                    ip
                } else {
                    return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                        "`ip` does not parse to IpAddr.",
                    )));
                };
                Ok(ip)
            }
            Some(_) => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`ip` does not map to string.",
                )))
            }
            None => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`ip` does not exist.",
                )))
            }
        }
    }

    // peers值可以是一个由 6 个字节的倍数组成的字符串。
    // 前 4 个字节是 IP 地址，后 2 个字节是端口号。全部采用网络（大端）表示法
    fn peers_from_bytes(&self, mut bytes: &[u8]) -> Result<Vec<SocketAddr>, TrackerError> {
        const ENTRY_LEN: usize = 6;
        let buf_len = bytes.len();
        if buf_len % ENTRY_LEN != 0 {
            return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                "peers compact string must be a multiple of 6",
            )));
        }

        let buf_len = bytes.len();
        let mut peers = Vec::with_capacity(buf_len * ENTRY_LEN);
        for _ in (0..buf_len).step_by(ENTRY_LEN) {
            let addr = Ipv4Addr::from(bytes.get_u32());
            let port = bytes.get_u16();
            let peer = SocketAddr::new(IpAddr::V4(addr), port);
            peers.push(peer);
        }
        Ok(peers)
    }

    fn peers(&self, dict: &mut Dict) -> Result<Vec<SocketAddr>, TrackerError> {
        match dict.remove("peers") {
            Some(BenObject::Bytes(ref bytes)) => self.peers_from_bytes(bytes),
            Some(BenObject::List(ref mut list)) => {
                let mut peers = Vec::with_capacity(list.len());
                for obj in list {
                    if let BenObject::Dict(dict) = obj {
                        let ip = self.ip(dict)?;
                        let port = self.port(dict)?;
                        peers.push(SocketAddr::new(ip, port));
                    }
                }
                Ok(peers)
            }
            Some(_) => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`peers` does not map to bytes or list.",
                )))
            }
            None => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`peers` does not exist.",
                )))
            }
        }
    }

    fn incomplete(&self, dict: &mut Dict) -> Result<Option<i64>, TrackerError> {
        match dict.remove("incomplete") {
            Some(BenObject::Int(incomplete)) => Ok(Some(incomplete)),
            Some(_) => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`incomplete` does not map to int.",
                )))
            }
            None => Ok(None),
        }
    }
    fn complete(&self, dict: &mut Dict) -> Result<Option<i64>, TrackerError> {
        match dict.remove("complete") {
            Some(BenObject::Int(complete)) => Ok(Some(complete)),
            Some(_) => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`complete` does not map to int.",
                )))
            }
            None => Ok(None),
        }
    }
    fn tracker_id(&self, dict: &mut Dict) -> Result<Option<String>, TrackerError> {
        match dict.remove("tracker id") {
            Some(BenObject::String(id)) => Ok(Some(id)),
            Some(_) => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`tracker id` does not map to string (or maps to invalid UTF8).",
                )))
            }
            None => Ok(None),
        }
    }

    fn min_interval(&self, dict: &mut Dict) -> Result<Option<Duration>, TrackerError> {
        match dict.remove("min interval") {
            Some(BenObject::Int(min_interval)) => {
                Ok(Some(Duration::from_secs(min_interval as u64)))
            }
            Some(_) => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`min interval` does not map to int.",
                )))
            }
            None => Ok(None),
        }
    }

    fn interval(&self, dict: &mut Dict) -> Result<Duration, TrackerError> {
        match dict.remove("interval") {
            Some(BenObject::Int(interval)) => Ok(Duration::from_secs(interval as u64)),
            Some(_) => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`interval` does not map to int.",
                )))
            }
            None => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`interval` does not exist.",
                )))
            }
        }
    }

    fn warning_message(&self, dict: &mut Dict) -> Result<Option<String>, TrackerError> {
        match dict.remove("warning_message") {
            Some(BenObject::String(warn)) => Ok(Some(warn)),
            Some(_) => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`warning_message` does not map to string (or maps to invalid UTF8).",
                )))
            }
            None => Ok(None),
        }
    }
    fn failure_reason(&self, dict: &mut Dict) -> Result<Option<String>, TrackerError> {
        match dict.remove("failure_reason") {
            Some(BenObject::String(reason)) => Ok(Some(reason)),
            Some(_) => {
                return Err(TrackerError::ParseResponseError(Cow::Borrowed(
                    "`failure_reason` does not map to string (or maps to invalid UTF8).",
                )))
            }
            None => Ok(None),
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
    use mockito::{mock, Matcher};
    use std::net::Ipv4Addr;

    use super::*;

    #[tokio::test]
    async fn test_mock_tracker_resp() {
        let addr = mockito::server_url();
        let tracker = Tracker::new(addr.parse().unwrap());

        let info_hash_str = "abcdefghij1234567890";
        let mut info_hash = [0; 20];
        info_hash.copy_from_slice(info_hash_str.as_bytes());

        let peer_id_str = "cbt-2020-03-03-00000";
        let mut peer_id = [0; 20];
        peer_id.copy_from_slice(peer_id_str.as_bytes());

        let req = Request {
            info_hash,
            peer_id,
            port: 16,
            downloaded: 1234,
            uploaded: 1234,
            left: 1234,
            compact: 1,
            no_peer_id: None,
            numwant: Some(1),
            ip: None,
            event: None,
            key: None,
            tracker_id: None,
        };
        let peer_ip = Ipv4Addr::new(2, 156, 201, 254);
        let peer_port = 49123;
        let expected = Response {
            tracker_id: None,
            failure_reason: None,
            warning_message: None,
            interval: Duration::from_secs(15),
            min_interval: Some(Duration::from_secs(10)),
            complete: Some(5),
            incomplete: Some(3),
            peers: vec![SocketAddr::new(peer_ip.into(), peer_port)],
        };

        let mut encoded_resp = Vec::new();
        // unterminated dict
        encoded_resp.extend_from_slice(
            b"d\
            8:completei5e\
            10:incompletei3e\
            8:intervali15e\
            12:min intervali10e",
        );
        // insert peers field into dict
        encoded_resp.extend_from_slice(b"5:peers");
        encoded_resp.extend_from_slice(&encode_compact_peers_list(&[(peer_ip, peer_port)]));
        // terminate dict
        encoded_resp.push(b'e');

        let _m = mock("GET", "/")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("compact".into(), "1".into()),
                Matcher::UrlEncoded("info_hash".into(), info_hash_str.into()),
                Matcher::UrlEncoded("peer_id".into(), peer_id_str.into()),
                Matcher::UrlEncoded("port".into(), req.port.to_string()),
                Matcher::UrlEncoded("downloaded".into(), req.downloaded.to_string()),
                Matcher::UrlEncoded("uploaded".into(), req.uploaded.to_string()),
                Matcher::UrlEncoded("left".into(), req.left.to_string()),
                Matcher::UrlEncoded("numwant".into(), req.numwant.unwrap().to_string()),
            ]))
            .with_status(200)
            .with_body(encoded_resp)
            .create();

        let actual = tracker.find_peers(req).await.unwrap();
        assert_eq!(actual, expected);
    }

    fn encode_compact_peers_list(peers: &[(Ipv4Addr, u16)]) -> Vec<u8> {
        let encoded_peers: Vec<_> = peers
            .iter()
            .map(|(ip, port)| {
                ip.octets()
                    .iter()
                    .chain([(port >> 8) as u8, (port & 0xff) as u8].iter())
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        let mut encoded = Vec::new();
        encoded.extend_from_slice(encoded_peers.len().to_string().as_bytes());
        encoded.push(b':');
        encoded.extend_from_slice(&encoded_peers);

        encoded
    }
}
