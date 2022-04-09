use std::io::{BufReader, Read};
use tracker::Request;

use torrent::{Info, TorrentFile};
use tracker::Tracker;

extern crate torrent;
extern crate tracker;

#[tokio::test]
async fn test_tracker_find_peers() {
    let file = std::fs::File::open("tests/files/debian-11.3.0-amd64-netinst.iso.torrent").unwrap();
    let mut bytes = Vec::new();
    BufReader::new(file).read_to_end(&mut bytes).unwrap();
    let parsed = TorrentFile::parse(bytes).unwrap();
    assert_eq!(
        parsed.announce,
        "http://bttracker.debian.org:6969/announce".to_owned()
    );
    assert_eq!(parsed.creation_date, Some(1648300186));
    assert_eq!(
        parsed.comment,
        Some(r#""Debian CD from cdimage.debian.org""#.to_owned())
    );
    let expect_info_hash: [u8; 20] = [
        177, 17, 129, 60, 230, 15, 66, 145, 151, 52, 130, 61, 245, 236, 32, 189, 30, 4, 231, 247,
    ];
    let info_hash = parsed.info.hash_bytes().unwrap();
    assert_eq!(info_hash, expect_info_hash);
    if let Info::SingleFile(single) = parsed.info {
        assert_eq!(single.name, "debian-11.3.0-amd64-netinst.iso".to_owned());
        assert_eq!(single.length, 396361728);
        assert_eq!(single.piece_length, 262144);
        let tracker = Tracker::new(parsed.announce.parse().unwrap());
        let resp = tracker
            .find_peers(Request {
                info_hash,
                peer_id: b"cbt-2022-03-03-00000".to_owned(),
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
        dbg!(resp);
    } else {
        panic!("not a single file torrent")
    }
}
