use thiserror::Error;

#[derive(Error, Debug)]
pub enum PeerError {
    #[error("wrong type")]
    WrongType,
    #[error("invalid bencode")]
    Invalid,
    #[error("EOF")]
    EOF,
    #[error(transparent)]
    IOError(#[from] ::std::io::Error),
    #[error("unknown bencode error")]
    Unknown,

    #[error(transparent)]
    StringUtf8Error(#[from] ::std::string::FromUtf8Error),

    #[error(transparent)]
    StrUtf8Error(#[from] ::std::str::Utf8Error),
}
