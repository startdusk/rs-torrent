use super::*;

impl BenObject {
	pub fn from_bytes<T>(bytes: T) -> Result<BenObject, BencodeError>
	where
		T: AsRef<[u8]>,
	{
		let mut buf = ByteBuffer::new(bytes.as_ref());
		Ok(BenObject::parse(&mut buf)?)
	}
	pub fn parse(r: &mut ByteBuffer) -> Result<BenObject, BencodeError> {
		match Self::peek_byte(r)? {
			DICT_PREFIX => {
				r.advance(1);
				// TODO: dict 必须是有序的？？？
				let mut dict = HashMap::new();
				loop {
					let b = r.peek().ok_or(BencodeError::EOF)?;
					if *b == DICT_POSTFIX {
						r.advance(1);
						break;
					}
					let key = match read_string(r)? {
						BenObject::String(k) => k,
						_ => return Err(BencodeError::ExpectStringError),
					};

					let val = Self::parse(r)?;
					dict.insert(key, val);
				}
				Ok(BenObject::Dict(dict))
			}
			LIST_PREFIX => {
				r.advance(1);
				let mut list = vec![];
				while Self::peek_byte(r)? != LIST_POSTFIX {
					list.push(Self::parse(r)?);
				}
				r.advance(1);
				Ok(BenObject::List(list))
			}
			INT_PREFIX => read_int(r),
			_ => read_string(r),
		}
	}

	fn peek_byte(bytes: &mut ByteBuffer) -> Result<u8, BencodeError> {
		match bytes.peek() {
			Some(&byte) => Ok(byte),
			None => Err(BencodeError::EOF),
		}
	}
}

// ANCHOR: decoder

fn read_string(r: &mut ByteBuffer) -> Result<BenObject, BencodeError> {
	let (num, len) = read_decimal(r)?;
	if len == 0 {
		return Err(BencodeError::ExpectNumberError(num, len));
	}

	let b = r.next().ok_or(BencodeError::EOF)?;
	if *b != STR_DELIMITER {
		return Err(BencodeError::ExpectColonError);
	}

	let mut string = String::with_capacity(num as usize);
	for _ in 0..num {
		let b = r.next().ok_or(BencodeError::EOF)?;
		string.push(char::from(*b));
	}

	Ok(BenObject::String(string))
}

// i333e 表示数字 333
fn read_int(r: &mut ByteBuffer) -> Result<BenObject, BencodeError> {
	// 去掉 'i'
	let b = r.next().ok_or(BencodeError::EOF)?;
	if *b != INT_PREFIX {
		return Err(BencodeError::ExpectCharIError);
	}
	// 读取数字
	let (val, _) = read_decimal(r)?;
	// 去掉 'e'
	let b = r.next().ok_or(BencodeError::EOF)?;
	if *b != INT_POSTFIX {
		return Err(BencodeError::ExpectCharEError);
	}

	Ok(BenObject::Int(val))
}

fn read_decimal(r: &mut ByteBuffer) -> Result<(i64, i64), BencodeError> {
	let mut sign = 1;
	let mut val = 0;
	let mut len = 0;
	let mut b = r.next().ok_or(BencodeError::EOF)?;
	len += 1;
	if *b == MINUS {
		sign = -1;
		b = r.next().ok_or(BencodeError::EOF)?;
		len += 1;
	}

	loop {
		if !check_num(*b) {
			r.push_back(1);
			len -= 1;
			return Ok((sign * val, len));
		}
		val = (val * 10) + (*b - ZERO) as i64;
		b = r.next().ok_or(BencodeError::EOF)?;
		len += 1;
	}
}
// ANCHOR_END: decoder

fn check_num(num: u8) -> bool {
	num >= b'0' && num <= b'9'
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashMap;

	#[test]
	fn test_read_string() {
		let cases = [("4:spam", "spam"), ("2:to", "to"), ("0:", "")];
		for cc in cases {
			let mut buf = ByteBuffer::new(cc.0.as_bytes());
			let obj = read_string(&mut buf).unwrap();
			if let BenObject::String(string) = obj {
				assert_eq!(string, cc.1);
			} else {
				panic!("expect bencode string!!!")
			}
		}
	}
	#[test]
	fn test_read_int() {
		let cases = [("i999e", 999), ("i0e", 0), ("i-99e", -99)];
		for cc in cases {
			let mut buf = ByteBuffer::new(cc.0.as_bytes());
			let obj = read_int(&mut buf).unwrap();
			assert_eq!(buf.pos(), cc.0.len());
			if let BenObject::Int(num) = obj {
				assert_eq!(num, cc.1);
			} else {
				panic!("expect bencode int!!!")
			}
		}
	}

	#[test]
	fn test_parse_list() {
		let source = "li123e6:archeri789ee";
		let mut buf = ByteBuffer::new(source.as_bytes());
		match BenObject::parse(&mut buf).unwrap() {
			BenObject::List(ref list) => {
				assert_eq!(list.len(), 3);
				match &list[0] {
					BenObject::Int(num) => {
						assert_eq!(*num, 123)
					}
					_ => panic!("expect list index 0 int"),
				};
				match &list[1] {
					BenObject::String(string) => {
						assert_eq!(string, "archer")
					}
					_ => panic!("expect list index 1 string"),
				}
				match &list[2] {
					BenObject::Int(num) => {
						assert_eq!(*num, 789)
					}
					_ => panic!("expect list index 1 string"),
				}
			}
			_ => panic!("expect bencode list"),
		}
	}

	#[test]
	fn test_parse_simple_dict() {
		let source = "d4:name3:ben3:agei29ee";
		let mut buf = ByteBuffer::new(source.as_bytes());
		match BenObject::parse(&mut buf).unwrap() {
			BenObject::Dict(ref dict) => {
				assert_eq!(dict.len(), 2);
				match &dict["name"] {
					BenObject::String(string) => {
						assert_eq!(string, "ben")
					}
					_ => panic!("expect dict key `name`"),
				};
				match &dict["age"] {
					BenObject::Int(num) => {
						assert_eq!(*num, 29)
					}
					_ => panic!("expect dict key `age`"),
				};
			}
			_ => panic!("expect bencode dict"),
		}
	}

	#[test]
	fn test_parse_complex_dict() {
		let source = "d4:userd4:name3:ben3:agei29ee5:valueli80ei85ei90eee";
		// { "user": { "name": "ben", "age": 29 }, "value": [80, 85, 90] }
		let mut buf = ByteBuffer::new(source.as_bytes());
		let user_val = HashMap::from([
			("name".to_string(), BenObject::String("ben".to_string())),
			("age".to_string(), BenObject::Int(29)),
		]);
		match BenObject::parse(&mut buf).unwrap() {
			BenObject::Dict(ref dict) => {
				assert_eq!(dict.len(), 2);
				match &dict["user"] {
					BenObject::Dict(dict) => {
						assert_eq!(dict, &user_val)
					}
					_ => panic!("expect dict key `user`"),
				};
				match &dict["value"] {
					BenObject::List(list) => {
						assert_eq!(
							list,
							&[BenObject::Int(80), BenObject::Int(85), BenObject::Int(90)]
						)
					}
					_ => panic!("expect dict value `age`"),
				};
			}
			_ => panic!("expect bencode dict"),
		}
	}
}
