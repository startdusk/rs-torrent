use thiserror::Error;

#[derive(Error, Debug)]
pub enum BencodeError {
    #[error("expect number")]
    ExpectNumberError,
    #[error("expect colon(:) but got {0}")]
    ExpectColonError(String),
    #[error("expect char i bot got {0}")]
    ExpectCharIError(u8),
    #[error("expect char e bot got {0}")]
    ExpectCharEError(u8),
    #[error("wrong type")]
    WrongType,
    #[error("invalid bencode")]
    Invalid,
    #[error("EOF")]
    EOF,
    #[error("unknown bencode error")]
    Unknown,
}
