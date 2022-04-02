use thiserror::Error;

#[derive(Error, Debug)]
pub enum TorrentError {
	#[error("torrent parse error: {0}")]
	ParseError(::std::borrow::Cow<'static, str>),
	#[error("torrent source code must be dict")]
	InvalidTorrent,
	#[error("torrent info error: {0}")]
	InvalidTorrentInfo(::std::borrow::Cow<'static, str>),
	#[error(transparent)]
	Utf8Error(#[from] ::std::string::FromUtf8Error),
	#[error(transparent)]
	BenObjectParseError(#[from] bencode::BencodeError),

	#[error(transparent)]
	IOError(#[from] ::std::io::Error),
	#[error("unknown torrent error")]
	Unknown,
}
