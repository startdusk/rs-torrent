use thiserror::Error;

#[derive(Error, Debug)]
pub enum BencodeError {
	#[error("string format prefix must be a number")]
	ExpectNumberError,
	#[error("expect string")]
	ExpectStringError,
	#[error("expect colon(:)")]
	ExpectColonError,
	#[error("expect char i")]
	ExpectCharIError,
	#[error("expect char e")]
	ExpectCharEError,
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
}
