use std::path::PathBuf;

pub mod error;
pub mod marshal;
pub mod parser;

pub use crate::error::TorrentError;

pub type Pieces = Vec<u8>;

pub type AnnounceList = Vec<Vec<String>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TorrentFile {
    pub info: Info,
    pub announce: String,
    pub announce_list: Option<AnnounceList>,
    pub creation_date: Option<String>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SingleFile {
    pub piece_length: i64,
    pub pieces: Pieces,
    pub private: Option<i64>,
    pub name: String,
    pub length: i64,
    pub md5sum: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MultipleFile {
    pub piece_length: i64,
    pub pieces: Pieces,
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
