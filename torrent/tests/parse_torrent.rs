use std::io::{BufReader, Read};

use torrent::{Info, TorrentFile};

extern crate torrent;

#[test]
fn test_parse_single_file_torrent() {
    let file = std::fs::File::open("tests/files/debian-iso.torrent").unwrap();
    let mut bytes = Vec::new();
    BufReader::new(file).read_to_end(&mut bytes).unwrap();

    let parsed = TorrentFile::parse(bytes).unwrap();
    assert_eq!(
        parsed.announce,
        "http://bttracker.debian.org:6969/announce".to_owned()
    );
    assert_eq!(parsed.creation_date, Some(1639833767));
    assert_eq!(
        parsed.comment,
        Some(r#""Debian CD from cdimage.debian.org""#.to_owned())
    );

    if let Info::SingleFile(single) = parsed.info {
        assert_eq!(single.name, "debian-11.2.0-amd64-netinst.iso".to_owned());
        assert_eq!(single.length, 396361728);
        assert_eq!(single.piece_length, 262144);
    } else {
        panic!("not a single file torrent")
    }
}
