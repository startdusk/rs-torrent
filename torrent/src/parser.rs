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
			BenObject::Dict(ref mut dict) => Ok(TorrentFile {
				info: Self::info(dict)?,
				announce: Self::announce(dict)?,
				announce_list: Self::announce_list(dict)?,
				creation_date: Self::creation_date(dict)?,
				comment: Self::comment(dict)?,
				created_by: Self::created_by(dict)?,
				encoding: Self::encoding(dict)?,
			}),
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

	fn creation_date(dict: &mut Dict) -> Result<Option<i64>, TorrentError> {
		match dict.remove("creation date") {
			Some(BenObject::Int(date)) => Ok(Some(date)),
			Some(_) => {
				return Err(TorrentError::ParseError(Cow::Borrowed(
					"`creation date` does not map to int.",
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
	fn test_torrent_single_file_parse() {
		let pieces: Vec<u8> = vec![
			0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
			0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
		];
		let pieces_str = std::str::from_utf8(pieces.as_slice()).unwrap();
		let bytes = benobject!({
			("info", {
				("piece length", 2),
				("pieces", pieces_str),
				("name", "startdusk"),
				("length", 2),
			}),
			("announce", "https://www.google.com"),
		})
		.bencode()
		.unwrap();

		assert_eq!(
			TorrentFile::parse(bytes).unwrap(),
			TorrentFile {
				info: Info::SingleFile(SingleFile {
					piece_length: 2,
					pieces,
					private: None,
					name: "startdusk".to_owned(),
					length: 2,
					md5sum: None,
				}),
				announce: "https://www.google.com".to_owned(),
				announce_list: None,
				creation_date: None,
				comment: None,
				created_by: None,
				encoding: None
			}
		);

		// d
		// 	8:announce
		// 		41:http://bttracker.debian.org:6969/announce
		// 	7:comment
		// 		35:"Debian CD from cdimage.debian.org"
		// 	13:creation date
		// 		i1573903810e
		// 	4:info
		// 		d
		// 			6:length
		// 				i351272960e
		// 			4:name
		// 				31:debian-10.2.0-amd64-netinst.iso
		// 			12:piece length
		// 				i262144e
		// 			6:pieces
		// 				39:binary blob of the hashes of each piece
		// 		e
		// e

		let bytes = r#"d8:announce41:http://bttracker.debian.org:6969/announce7:comment35:"Debian CD from cdimage.debian.org"13:creation datei1573903810e4:infod6:lengthi351272960e4:name31:debian-10.2.0-amd64-netinst.iso12:piece lengthi262144e6:pieces39:binary blob of the hashes of each pieceee"#;
		assert_eq!(
			TorrentFile::parse(bytes).unwrap(),
			TorrentFile {
				info: Info::SingleFile(SingleFile {
					piece_length: 262144,
					pieces: "binary blob of the hashes of each piece"
						.as_bytes()
						.to_vec(),
					private: None,
					name: "debian-10.2.0-amd64-netinst.iso".to_owned(),
					length: 351272960,
					md5sum: None,
				}),
				announce: "http://bttracker.debian.org:6969/announce".to_owned(),
				announce_list: None,
				creation_date: Some(1573903810),
				comment: Some(r#""Debian CD from cdimage.debian.org""#.to_owned()),
				created_by: None,
				encoding: None
			}
		);
	}

	#[test]
	fn test_torrent_multiple_file_parse() {
		let pieces: Vec<u8> = vec![
			0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
			0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
		];
		let pieces_str = std::str::from_utf8(pieces.as_slice()).unwrap();
		let bytes = benobject!({
			("info", {
				("name", "startdusk"),
				("piece length", 2),
				("pieces", pieces_str),
				("files", [
					{
						("length", 512),
						("path", ["a", "b", "c", "d.txt"])
					},
					{
						("length", 1024),
						("path", ["a", "b", "c", "f.txt"])
					}
				]),
			}),
			("announce", "https://www.google.com"),
		})
		.bencode()
		.unwrap();

		assert_eq!(
			TorrentFile::parse(bytes).unwrap(),
			TorrentFile {
				info: Info::MultipleFile(MultipleFile {
					name: "startdusk".to_owned(),
					piece_length: 2,
					pieces,
					private: None,
					files: vec![
						File {
							length: 512,
							md5sum: None,
							path: PathBuf::from(r"a/b/c/d.txt")
						},
						File {
							length: 1024,
							md5sum: None,
							path: PathBuf::from(r"a/b/c/f.txt")
						}
					]
				}),
				announce: "https://www.google.com".to_owned(),
				announce_list: None,
				creation_date: None,
				comment: None,
				created_by: None,
				encoding: None
			}
		);
	}
}
