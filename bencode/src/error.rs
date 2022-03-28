use thiserror::Error;

#[derive(Error, Debug)]
pub enum BencodeError {
    #[error("expect number but got {0}")]
    ErrNumber(i64),
    #[error("expect colon(:) but got {0}")]
    ErrColon(String),
    #[error("expect char i bot got {0}")]
    ErrEpI(String),
    #[error("expect char e bot got {0}")]
    ErrEpE(String),
    #[error("wrong type")]
    ErrTyp,
    #[error("invalid bencode")]
    Invalid,
    #[error("unknown bencode error")]
    Unknown,
}
