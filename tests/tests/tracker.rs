use std::io::{BufReader, Read};

use torrent::{Info, TorrentFile};
use tracker::Tracker;

extern crate torrent;
extern crate tracker;

#[tokio::test]
async fn test_tracker_build_url_valid() {
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
    let hash: Vec<u8> = vec![
        177, 17, 129, 60, 230, 15, 66, 145, 151, 52, 130, 61, 245, 236, 32, 189, 30, 4, 231, 247,
    ];
    let info_hash = parsed.info.hash_bytes().unwrap();
    if let Info::SingleFile(single) = parsed.info {
        assert_eq!(single.name, "debian-11.3.0-amd64-netinst.iso".to_owned());
        assert_eq!(single.length, 396361728);
        assert_eq!(single.piece_length, 262144);
        // let url = Tracker::build_url(
        //     parsed.announce,
        //     tracker::Request {
        //         info_hash,
        //         peer_id: vec![
        //             1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        //         ],
        //         port: 6881,
        //         uploaded: 0,
        //         downloaded: 0,
        //         left: single.length as usize,
        //         compact: 1,
        //         no_peer_id: None,
        //         event: None,
        //         ip: None,
        //         numwant: None,
        //         key: None,
        //         tracker_id: None,
        //     },
        // )
        // .unwrap();

        // let resp = reqwest::get(dbg!(url)).await.unwrap();
        // println!("{:#?}", resp);
    } else {
        panic!("not a single file torrent")
    }
}

// %B1%11%81%3C%E6%0FB%91%974%82%3D%F5%EC%20%BD%1E%04%E7%F7
