use std::io::{BufReader, Read};
use std::time::Duration;
use tracker::Request;

use torrent::{Info, TorrentFile};
use tracker::Tracker;

extern crate torrent;
extern crate tracker;

#[tokio::test]
async fn find_peers_from_single_file_torrent() {
    let file = std::fs::File::open("tests/files/debian-11.3.0-amd64-netinst.iso.torrent").unwrap();
    let mut bytes = Vec::new();
    BufReader::new(file).read_to_end(&mut bytes).unwrap();
    let parsed = TorrentFile::parse(bytes).unwrap();
    assert_eq!(
        parsed.announce,
        "http://bttracker.debian.org:6969/announce".to_owned()
    );
    assert_eq!(parsed.announce_list, None);
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
                left: single.length,
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
        assert_eq!(resp.interval, Duration::from_secs(900));

        assert_eq!(resp.peers.len(), 50);
        dbg!(resp);
    } else {
        panic!("not a single file torrent")
    }
}

#[tokio::test]
async fn find_peers_from_multiple_file_torrent() {
    // TODO: 国内种子发现没有announce
    let file =
        std::fs::File::open("tests/files/MP3-daily-2022-April-02-Electronic-[rarbg.to].torrent")
            .unwrap();
    let mut bytes = Vec::new();
    BufReader::new(file).read_to_end(&mut bytes).unwrap();

    let parsed = TorrentFile::parse(bytes).unwrap();
    // assert_eq!(
    //     parsed.info.hash_string().unwrap(),
    //     "0C34791F492A1B20580E61F639715A6FB0F8963E".to_owned()
    // );
    assert_eq!(
        parsed.announce,
        "http://tracker.trackerfix.com:80/announce".to_owned()
    );
    // assert_eq!(parsed.creation_date, Some(1430114167));
    // assert_eq!(parsed.comment, None);
    // assert_eq!(parsed.encoding, Some("UTF-8".to_string()));
    // assert_eq!(parsed.created_by, Some("BitTorrent/7800".to_owned()));
    // assert_eq!(
    //     parsed.announce_list,
    //     Some(vec![
    //         vec!["http://172.31.1.50:1313/announce".to_owned()],
    //         vec!["http://res.nana.hdq.me:1313/announce".to_owned()],
    //     ])
    // );
    let info_hash = parsed.info.hash_bytes().unwrap();
    if let Info::MultipleFile(multiple) = parsed.info {
        // assert_eq!(multiple.piece_length, 8388608);
        // assert_eq!(multiple.name, "行尸走肉2".to_owned());
        // assert_eq!(multiple.files.len(), 13);
        let mut total_length = 0;
        for file in multiple.files {
            total_length += file.length;
        }
        let tracker = Tracker::new(parsed.announce.parse().unwrap());
        let resp = tracker
            .find_peers(Request {
                info_hash,
                peer_id: b"cbt-2022-03-03-00000".to_owned(),
                port: 6881,
                uploaded: 0,
                downloaded: 0,
                left: total_length,
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
        dbg!(&resp);
        assert_eq!(resp.interval, Duration::from_secs(900));

        assert_eq!(resp.peers.len(), 50);
    } else {
        panic!("not a multiple file torrent")
    }
}
