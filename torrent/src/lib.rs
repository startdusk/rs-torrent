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
    pub fn hash(&self) -> Result<String, TorrentError> {
        let mut hasher = Sha1::new();
        let mut map: Dict = HashMap::new();
        match *self {
            Self::SingleFile(ref signle) => {
                hasher.update(benobject!({}).bencode()?);
                let output = hasher.finalize();
                // TODO: convert to string
                Ok("".to_owned())
            }
            Self::MultipleFile(ref multiple) => {
                panic!()
            }
        }
    }
}
