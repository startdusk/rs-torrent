use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrackerError {
    #[error("invalid response")]
    InvalidResponse,

	#[error("response parse error: {0}")]
	ParseResponseError(::std::borrow::Cow<'static, str>),
    #[error(transparent)]
    BenObjectParseError(#[from] bencode::BencodeError),
    #[error(transparent)]
    RequestError(#[from] ::reqwest::Error),
    #[error("unknown torrent error")]
    Unknown,
}
