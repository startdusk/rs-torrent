use std::{
    io::{BufReader, Read},
    path::PathBuf,
};

use torrent::{File, Info, TorrentFile};

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
    let hash: [u8; 20] = [
        40, 197, 81, 150, 245, 119, 83, 196, 10, 206, 182, 251, 88, 97, 126, 105, 149, 167, 237,
        219,
    ];
    let info_hash = parsed.info.hash_bytes().unwrap();
    assert_eq!(info_hash, hash);
    if let Info::SingleFile(single) = parsed.info {
        assert_eq!(single.name, "debian-11.2.0-amd64-netinst.iso".to_owned());
        assert_eq!(single.length, 396361728);
        assert_eq!(single.piece_length, 262144);
    } else {
        panic!("not a single file torrent")
    }
}

#[test]
fn test_parse_multiple_file_torrent() {
    let file =
        std::fs::File::open("tests/files/MP3-daily-2022-April-02-Electronic-[rarbg.to].torrent")
            .unwrap();
    let mut bytes = Vec::new();
    BufReader::new(file).read_to_end(&mut bytes).unwrap();

    let parsed = TorrentFile::parse(bytes).unwrap();

    assert_eq!(
        parsed.info.hash_string().unwrap(),
        "E9CE30881B905B809EC094BCB2F17D202627F05F".to_owned()
    );
    assert_eq!(
        parsed.announce,
        "http://tracker.trackerfix.com:80/announce".to_owned()
    );
    assert_eq!(parsed.creation_date, Some(1648983368));
    assert_eq!(
        parsed.comment,
        Some(r#"Torrent downloaded from https://rarbg.to"#.to_owned())
    );
    assert_eq!(parsed.created_by, Some("RARBG".to_owned()));
    assert_eq!(parsed.creation_date, Some(1648983368));
    assert_eq!(
        parsed.announce_list,
        Some(vec![
            vec!["http://tracker.trackerfix.com:80/announce".to_owned()],
            vec!["udp://9.rarbg.me:2770/announce".to_owned()],
            vec!["udp://9.rarbg.to:2800/announce".to_owned()],
            vec!["udp://tracker.fatkhoala.org:13760/announce".to_owned()],
            vec!["udp://tracker.thinelephant.org:12750/announce".to_owned()]
        ])
    );

    let file =
        std::fs::File::open("tests/files/MP3-daily-2022-April-02-Pop-Folk-[rarbg.to].torrent")
            .unwrap();
    let mut bytes = Vec::new();
    BufReader::new(file).read_to_end(&mut bytes).unwrap();

    let parsed = TorrentFile::parse(bytes).unwrap();
    assert_eq!(
        parsed.info.hash_string().unwrap(),
        "69BAFA13168FBCD6961A67B83EE36899C29B33F0".to_owned()
    );
    assert_eq!(
        parsed.announce,
        "http://tracker.trackerfix.com:80/announce".to_owned()
    );
    assert_eq!(parsed.creation_date, Some(1648983339));
    assert_eq!(
        parsed.comment,
        Some(r#"Torrent downloaded from https://rarbg.to"#.to_owned())
    );
    assert_eq!(parsed.created_by, Some("RARBG".to_owned()));
    assert_eq!(parsed.creation_date, Some(1648983339));
    assert_eq!(
        parsed.announce_list,
        Some(vec![
            vec!["http://tracker.trackerfix.com:80/announce".to_owned()],
            vec!["udp://9.rarbg.me:2950/announce".to_owned()],
            vec!["udp://9.rarbg.to:2850/announce".to_owned()],
            vec!["udp://tracker.thinelephant.org:12800/announce".to_owned()],
            vec!["udp://tracker.fatkhoala.org:13710/announce".to_owned()]
        ])
    );

    if let Info::MultipleFile(multiple) = parsed.info {
        assert_eq!(multiple.piece_length, 1048576);
        assert_eq!(multiple.name, "MP3-daily-2022-April-02-Pop-Folk".to_owned());
        assert_eq!(multiple.files.len(), 15);
        assert_eq!(
            multiple.files,
            vec![
                File {
                    length: 193445,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/00_filzmooser-ein_herz_voll_musik-web-de-2006.jpg"
                    )
                },
                File {
                    length: 430,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/00_filzmooser-ein_herz_voll_musik-web-de-2006.m3u"
                    )
                },
                File {
                    length: 1771,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/00_filzmooser-ein_herz_voll_musik-web-de-2006.nfo"
                    )
                },
                File {
                    length: 8491981,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/01_filzmooser_-_costa_brava.mp3"
                    )
                },
                File {
                    length: 6529662,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/02_filzmooser_-_karibik_faszination.mp3"
                    )
                },
                File {
                    length: 7435589,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/03_filzmooser_-_mariella.mp3"
                    )
                },
                File {
                    length: 7353042,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/04_filzmooser_-_in_der_ferne.mp3"
                    )
                },
                File {
                    length: 5453418,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/05_filzmooser_-_i_wait_for_you.mp3"
                    )
                },
                File {
                    length: 8535867,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/06_filzmooser_-_guten_tag_sonne.mp3"
                    )
                },
                File {
                    length: 7197352,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/07_filzmooser_-_ein_herz_voll_musik.mp3"
                    )
                },
                File {
                    length: 6404275,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/08_filzmooser_-_haymos_dance.mp3"
                    )
                },
                File {
                    length: 7979981,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/09_filzmooser_-_save_your_love.mp3"
                    )
                },
                File {
                    length: 7909973,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/10_filzmooser_-_hula_cha_cha.mp3"
                    )
                },
                File {
                    length: 7418914,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/11_filzmooser_-_nicht_jeder_tag_bringt_sonnenschein.mp3"
                    )
                },
                File {
                    length: 7000911,
                    md5sum: None,
                    path: PathBuf::from(
                        r"Filzmooser-Ein_Herz_Voll_Musik-WEB-DE-2006-ALPMP3/12_filzmooser_-_pegasus.mp3"
                    )
                },
            ]
        )
    } else {
        panic!("not a multiple file torrent")
    }
}
