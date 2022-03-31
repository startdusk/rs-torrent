use std::borrow::Cow;

use super::*;

use bencode::{BenObject, Dict};

impl TorrentFile {
	pub fn parse<T>(bytes: T) -> Result<TorrentFile, TorrentError>
	where
		T: AsRef<[u8]>,
	{
		let mut obj = BenObject::from_bytes(bytes)?;
		match obj {
			BenObject::Dict(ref mut dict) => match dict.remove("info") {
				Some(BenObject::Dict(ref mut info)) => Ok(TorrentFile {
					info: Self::info(info)?,
					announce: Self::announce(dict)?,
					announce_list: Self::announce_list(dict)?,
					creation_date: Self::creation_date(dict)?,
					comment: Self::comment(dict)?,
					created_by: Self::created_by(dict)?,
					encoding: Self::encoding(dict)?,
				}),
				Some(_) => {
					return Err(TorrentError::ParseError(Cow::Borrowed(
						"`info` is not a dict.",
					)))
				}
				None => {
					return Err(TorrentError::ParseError(Cow::Borrowed(
						"`info` does not exist.",
					)))
				}
			},
			_ => return Err(TorrentError::InvalidTorrent),
		}
	}

	fn announce(dict: &mut Dict) -> Result<String, TorrentError> {
		match dict.remove("announce") {
			Some(BenObject::String(url)) => Ok(url),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`announce` does not map to string (or maps to invalid UTF8).",
				)))
			}
			None => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`announce` does not exist.",
				)))
			}
		}
	}

	fn creation_date(dict: &mut Dict) -> Result<Option<String>, TorrentError> {
		match dict.remove("creation date") {
			Some(BenObject::String(date)) => Ok(Some(date)),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`creation date` does not map to string (or maps to invalid UTF8).",
				)))
			}
			None => Ok(None),
		}
	}

	fn comment(dict: &mut Dict) -> Result<Option<String>, TorrentError> {
		match dict.remove("comment") {
			Some(BenObject::String(comment)) => Ok(Some(comment)),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`comment` does not map to string (or maps to invalid UTF8).",
				)))
			}
			None => Ok(None),
		}
	}

	fn created_by(dict: &mut Dict) -> Result<Option<String>, TorrentError> {
		match dict.remove("created by") {
			Some(BenObject::String(created)) => Ok(Some(created)),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`created by` does not map to string (or maps to invalid UTF8).",
				)))
			}
			None => Ok(None),
		}
	}

	fn encoding(dict: &mut Dict) -> Result<Option<String>, TorrentError> {
		match dict.remove("encoding") {
			Some(BenObject::String(encoding)) => Ok(Some(encoding)),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`encoding` does not map to string (or maps to invalid UTF8).",
				)))
			}
			None => Ok(None),
		}
	}

	fn announce_list(dict: &mut Dict) -> Result<Option<AnnounceList>, TorrentError> {
		match dict.remove("announce-list") {
			Some(BenObject::List(list)) => {
				let mut announces = Vec::new();
				for obj in list {
					announces.push(Self::announce_list_sub(obj)?);
				}
				Ok(Some(announces))
			}
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`announce-list` does not map to a list.",
				)))
			}
			None => Ok(None),
		}
	}

	fn announce_list_sub(obj: BenObject) -> Result<Vec<String>, TorrentError> {
		match obj {
			BenObject::List(list) => {
				let mut urls = Vec::new();
				for obj in list {
					match obj {
						BenObject::String(url) => urls.push(url),
						_ => {
							return Err(TorrentError::ParseError(Cow::Borrowed(
								"`announce-list` element is not a string.",
							)))
						}
					}
				}
				Ok(urls)
			}
			_ => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`announce-list` does not have any element.",
				)))
			}
		}
	}

	fn name(dict: &mut Dict) -> Result<String, TorrentError> {
		match dict.remove("name") {
			Some(BenObject::String(name)) => Ok(name),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`name` does not map to a string (or maps to invalid UTF8).",
				)))
			}
			None => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`name` does not exist.",
				)))
			}
		}
	}

	fn piece_length(dict: &mut Dict) -> Result<i64, TorrentError> {
		match dict.remove("piece length") {
			Some(BenObject::Int(len)) => Ok(len),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`piece length` does not map to a string (or maps to invalid UTF8).",
				)))
			}
			None => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`piece length` does not exist.",
				)))
			}
		}
	}

	fn length(dict: &mut Dict) -> Result<i64, TorrentError> {
		match dict.remove("length") {
			Some(BenObject::Int(len)) => Ok(len),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`length` does not map to a int.",
				)))
			}
			None => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`length` does not exist.",
				)))
			}
		}
	}

	fn md5sum(dict: &mut Dict) -> Result<Option<String>, TorrentError> {
		match dict.remove("md5sum") {
			Some(BenObject::String(sum)) => Ok(Some(sum)),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`md5sum` does not map to a string (or maps to invalid UTF8).",
				)))
			}
			None => Ok(None),
		}
	}

	fn path(dict: &mut Dict) -> Result<PathBuf, TorrentError> {
		match dict.remove("path") {
			Some(BenObject::List(ps)) => {
				let mut pb = PathBuf::new();
				for p in ps {
					if let BenObject::String(pair) = p {
						pb.push(pair);
					} else {
						return Err(TorrentError::ParseError(Cow::Borrowed("")));
					}
				}
				Ok(pb)
			}
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`path` does not map to a list.",
				)))
			}
			None => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`path` does not exist.",
				)))
			}
		}
	}

	fn files(obj: &mut BenObject) -> Result<Vec<File>, TorrentError> {
		match obj {
			BenObject::List(ref mut list) => {
				let mut files = Vec::with_capacity(list.len());
				for file in list {
					if let BenObject::Dict(ref mut dict) = file {
						files.push(File {
							length: Self::length(dict)?,
							md5sum: Self::md5sum(dict)?,
							path: Self::path(dict)?,
						});
					} else {
						return Err(TorrentError::ParseError(Cow::Borrowed("")));
					}
				}
				Ok(files)
			}
			_ => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"multiple file field `files` is not a list",
				)))
			}
		}
	}

	fn pieces(dict: &mut Dict) -> Result<Pieces, TorrentError> {
		match dict.remove("pieces") {
			Some(BenObject::String(p)) => Ok(p.as_bytes().to_vec()),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`pieces` does not map to a int.",
				)))
			}
			None => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`pieces` does not exist.",
				)))
			}
		}
	}

	fn private(dict: &mut Dict) -> Result<Option<i64>, TorrentError> {
		match dict.remove("private") {
			Some(BenObject::Int(p)) => Ok(Some(p)),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`private` does not map to a int.",
				)))
			}
			None => Ok(None),
		}
	}

	fn info(dict: &mut Dict) -> Result<Info, TorrentError> {
		match dict.remove("info") {
			Some(BenObject::Dict(ref mut info)) => {
				let name = Self::name(info)?;
				let piece_length = Self::piece_length(info)?;
				let pieces = Self::pieces(info)?;
				let private = Self::private(info)?;
				if let Some(ref mut files) = info.remove("files") {
					Ok(Info::MultipleFile(MultipleFile {
						piece_length,
						pieces,
						private,
						name,
						files: Self::files(files)?,
					}))
				} else {
					Ok(Info::SingleFile(SingleFile {
						piece_length,
						pieces,
						private,
						name,
						length: Self::length(info)?,
						md5sum: Self::md5sum(info)?,
					}))
				}
			}
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`info` is not a dict.",
				)))
			}
			None => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`info` does not exist.",
				)))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_work() {}
}
