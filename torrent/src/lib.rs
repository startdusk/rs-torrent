use std::{collections::HashMap, path::PathBuf};

use bencode::{BenObject, Dict};
use sha1::{Digest, Sha1};

#[macro_use]
extern crate bencode;

mod error;
mod marshal;
mod parser;

pub use crate::error::TorrentError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TorrentFile {
    pub info: Info,
    pub announce: String,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<i64>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SingleFile {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: Option<i64>,
    pub name: String,
    pub length: i64,
    pub md5sum: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MultipleFile {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: Option<i64>,
    pub name: String,
    pub files: Vec<File>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct File {
    pub length: i64,
    pub md5sum: Option<String>,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Info {
    SingleFile(SingleFile),
    MultipleFile(MultipleFile),
}

pub type Sha1Hash = [u8; 20];

impl Info {
    pub fn hash_bytes(&self) -> Result<Sha1Hash, TorrentError> {
        let output = self.marshal()?;
        let digest = Sha1::digest(&output);
        let mut info_hash = [0; 20];
        info_hash.copy_from_slice(&digest);
        Ok(info_hash)
    }
    pub fn hash_string(&self) -> Result<String, TorrentError> {
        let output = self.marshal()?;
        Ok(format!("{:X}", Sha1::digest(output)))
    }

    fn marshal(&self) -> Result<Vec<u8>, TorrentError> {
        let mut map: Dict = HashMap::new();
        let output = match *self {
            Self::SingleFile(ref single) => {
                map.insert(
                    "piece length".to_owned(),
                    BenObject::Int(single.piece_length),
                );
                map.insert("pieces".to_owned(), BenObject::Bytes(single.pieces.clone()));
                if let Some(private) = single.private {
                    map.insert("private".to_owned(), BenObject::Int(private));
                }
                map.insert("name".to_owned(), BenObject::String(single.name.clone()));
                map.insert("length".to_owned(), BenObject::Int(single.length));
                if let Some(md5sum) = &single.md5sum {
                    map.insert("md5sum".to_owned(), BenObject::String(md5sum.clone()));
                }
                BenObject::Dict(map).bencode()?
            }
            Self::MultipleFile(ref multiple) => {
                map.insert(
                    "piece length".to_owned(),
                    BenObject::Int(multiple.piece_length),
                );
                map.insert(
                    "pieces".to_owned(),
                    BenObject::Bytes(multiple.pieces.clone()),
                );
                if let Some(private) = multiple.private {
                    map.insert("private".to_owned(), BenObject::Int(private));
                }
                map.insert("name".to_owned(), BenObject::String(multiple.name.clone()));
                let mut files = Vec::new();
                for file in &multiple.files {
                    let mut fmap: Dict = HashMap::new();
                    fmap.insert("length".to_owned(), BenObject::Int(file.length));
                    if let Some(md5sum) = &file.md5sum {
                        fmap.insert("md5sum".to_owned(), BenObject::String(md5sum.clone()));
                    }

                    fmap.insert(
                        "path".to_owned(),
                        BenObject::List(
                            file.path
                                .iter()
                                .map(|component| {
                                    BenObject::String(component.to_string_lossy().into_owned())
                                })
                                .collect(),
                        ),
                    );
                    files.push(BenObject::Dict(fmap));
                }
                map.insert("files".to_owned(), BenObject::List(files));
                BenObject::Dict(map).bencode()?
            }
        };
        Ok(output)
    }
}

#[cfg(test)]
mod test_info {
    use super::*;

    #[test]
    fn test_into_single_file_hash() {
        let info = Info::SingleFile(SingleFile {
            piece_length: 100,
            pieces: vec![1, 2],
            private: Some(0),
            name: "startdusk".to_owned(),
            length: 100,
            md5sum: Some("todo!()".to_owned()),
        });

        assert_eq!(
            info.hash_string().unwrap(),
            "6067089A615467A306BE088FEB301AF27B4CE034".to_owned()
        );
    }
    #[test]
    fn test_into_multiple_file_hash() {
        let shash = Info::MultipleFile(MultipleFile {
            piece_length: 262144,
            pieces: vec![1, 2],
            private: Some(1),
            name: "debian-10.2.0-amd64-netinst.iso".to_owned(),
            files: vec![
                File {
                    length: 512,
                    md5sum: Some("14e1b600b1fd579f47433b88e8d85291132".to_owned()),
                    path: PathBuf::from(r"a/b/c/d.txt"),
                },
                File {
                    length: 1024,
                    md5sum: Some("1d4bbcfed31c6e01e90d8e4099e39eb7".to_owned()),
                    path: PathBuf::from(r"a/b/c/f.txt"),
                },
            ],
        })
        .hash_string()
        .unwrap();

        assert_eq!(shash, "57EFD09D0E3C07FC983DFC2A7303A81556272A21".to_owned());
    }
}
