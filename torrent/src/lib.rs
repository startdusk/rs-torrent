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
    pub pieces: String,
    pub private: Option<i64>,
    pub name: String,
    pub length: i64,
    pub md5sum: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MultipleFile {
    pub piece_length: i64,
    pub pieces: String,
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

impl Info {
    pub fn hash_bytes(&self) -> Result<Vec<u8>, TorrentError> {
        let output = self.marshal()?;
        Ok(Sha1::digest(output).to_vec())
    }
    pub fn hash_string(&self) -> Result<String, TorrentError> {
        let output = self.marshal()?;
        Ok(format!("{:x}", Sha1::digest(output)))
    }

    fn marshal(&self) -> Result<Vec<u8>, TorrentError> {
        let mut map: Dict = HashMap::new();
        let output = match *self {
            Self::SingleFile(ref single) => {
                map.insert(
                    "piece length".to_owned(),
                    BenObject::Int(single.piece_length),
                );
                map.insert(
                    "pieces".to_owned(),
                    BenObject::String(single.pieces.clone()),
                );
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
                    BenObject::String(multiple.pieces.clone()),
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
        let shash = Info::SingleFile(SingleFile {
            piece_length: 100,
            pieces: "1d4bbcfed31c6e01e90d8e4099e39eb7".to_owned(),
            private: Some(0),
            name: "startdusk".to_owned(),
            length: 100,
            md5sum: Some("todo!()".to_owned()),
        })
        .hash_string()
        .unwrap();

        assert_eq!(shash, "b2a6a322290f3f8b146490501898cccb12f66f63".to_owned());
    }
    #[test]
    fn test_into_multiple_file_hash() {
        let shash = Info::MultipleFile(MultipleFile {
            piece_length: 262144,
            pieces: "binary blob of the hashes of each piece".to_owned(),
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

        assert_eq!(shash, "330c0cca65ddcf93bafdc3fdf02a7c394bd5829d".to_owned());
    }
}
