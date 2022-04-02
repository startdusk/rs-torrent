use super::*;

use std::collections::HashMap;
use std::hash::BuildHasher;
use std::io::Write;

impl BenObject {
	pub fn bencode(&self) -> Result<Vec<u8>, BencodeError> {
		let mut w = Vec::new();
		let _ = self.write_into(&mut w)?;
		Ok(w)
	}

	pub fn write_into<W>(&self, w: &mut W) -> Result<usize, BencodeError>
	where
		W: Write,
	{
		let mut wlen = 0;
		match *self {
			BenObject::Int(num) => wlen += Self::write_int(w, num)?,
			BenObject::String(ref s) => wlen += Self::write_string(w, s)?,
			BenObject::List(ref list) => wlen += Self::write_list(w, list)?,
			BenObject::Dict(ref dict) => wlen += Self::write_dict(w, dict)?,
		}

		Ok(wlen)
	}

	// ANCHOR: encoder
	// 列表的格式为 l不限数量个BE量e（小写L开头）
	// l4:spam4:eggse 表示 [ "spam", "eggs" ]
	// li123e5:helloi111ee 表示 [ 123, "hello", 111 ]
	// le 表示 [] 空列表
	fn write_list<W, L>(w: &mut W, list: L) -> Result<usize, BencodeError>
	where
		W: Write,
		L: AsRef<[BenObject]>,
	{
		let mut wlen = 0;
		let list = list.as_ref();
		w.write_all(&[LIST_PREFIX])?;
		wlen += 1;
		for item in list {
			wlen += item.write_into(w)?;
		}
		w.write_all(&[LIST_POSTFIX])?;
		wlen += 1;
		Ok(wlen)
	}

	// 字典的格式为 d不限数量个字段e(小写D开头)
	// 字段 是指一种 key-value 结构，其中 key 是一个BE字节串，一个字段的格式为 一个BE字节串+一个BE量
	// d3:cow3:moo4:spam4:eggse 表示 {"cow":"moo", "spam": "egge"}
	// d4:name5:Angus3:agei23ee 表示 {"name":"Angus", "age":23}
	// d4:spaml1:a1:bee 表示 {"spam":["a","b"]}
	// de 表示 {}
	// 注意：key 是 BE字节串，而不是字符串，因此 key 的比较是二进制比较而不是字符串比较
	fn write_dict<W, S>(
		w: &mut W,
		dict: &HashMap<String, BenObject, S>,
	) -> Result<usize, BencodeError>
	where
		W: Write,
		S: BuildHasher,
	{
		let mut sorted = dict.iter().collect::<Vec<(&String, &BenObject)>>();
		sorted.sort_by_key(|&(key, _)| key.as_bytes());
		let mut wlen = 0;
		w.write_all(&[DICT_PREFIX])?;
		wlen += 1;
		for (key, val) in sorted {
			wlen += Self::write_string(w, key)?;
			wlen += val.write_into(w)?;
		}
		w.write_all(&[DICT_POSTFIX])?;
		wlen += 1;
		Ok(wlen)
	}

	// 字节串的格式为 字节串长度:内容，其中 字节串长度 是 ASCII 编码格式的整数字符串，单位为字节
	// 4:abcd 表示4个字节的串 "abcd"
	// 0:     表示0个字节的串 ""
	fn write_string<W, S>(w: &mut W, s: S) -> Result<usize, BencodeError>
	where
		W: Write,
		S: AsRef<str>,
	{
		let s = s.as_ref();
		let slen = s.len();
		let mut wlen = Self::write_decimal(w, slen as i64)?;
		w.write_all(&[STR_DELIMITER])?;
		wlen += 1;
		w.write_all(s.as_bytes())?;
		wlen += slen;
		w.flush()?;
		Ok(wlen)
	}

	// 整数的格式为 i整数e，其中 整数 是 ASCII 编码格式的整数字符串
	// i1234e 表示整数 1234
	// 注意：
	//      1.i-0e 是无效编码
	//      2.除了 i0e 之外，一切以0开头的整数如 i03e, i011e 都是无效的编码
	//      3.虽然并未规定整数类型的最大值，但是 64位 整数的支持是强制的、必不可少的，以支持超过 4GB 大小的文件
	fn write_int<W>(w: &mut W, val: i64) -> Result<usize, BencodeError>
	where
		W: Write,
	{
		let mut wlen = 0;
		w.write_all(&[INT_PREFIX])?;
		wlen += 1;
		let nlen = Self::write_decimal(w, val)?;
		wlen += nlen;
		w.write_all(&[INT_POSTFIX])?;
		wlen += 1;

		w.flush()?;
		Ok(wlen)
	}

	fn write_decimal<W>(w: &mut W, val: i64) -> Result<usize, BencodeError>
	where
		W: Write,
	{
		let mut val = val;
		let mut len = 0;
		if val == 0 {
			w.write_all(&[ZERO])?;
			len += 1;
			return Ok(len);
		}

		if val < 0 {
			w.write_all(&[MINUS])?;
			len += 1;
			val *= -1;
		}

		let mut dividend = 1;
		loop {
			if dividend > val {
				dividend /= 10;
				break;
			}
			dividend *= 10;
		}

		loop {
			w.write_fmt(format_args!("{}", (val / dividend)))?;
			len += 1;
			if dividend == 1 {
				return Ok(len);
			}
			val %= dividend;
			dividend /= 10;
		}
	}
	// ANCHOR_END: encoder
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bencode_int() {
		let cases = [(999, 5, "i999e"), (0, 3, "i0e"), (-99, 5, "i-99e")];
		for cc in cases {
			let vec = BenObject::Int(cc.0).bencode().unwrap();
			assert_eq!(vec.len(), cc.1);
			assert_eq!(vec, cc.2.as_bytes().to_vec());
		}
	}

	#[test]
	fn test_bencode_string() {
		let cases = [
			("spam".to_string(), 6, "4:spam"),
			("to".to_string(), 4, "2:to"),
			("".to_string(), 2, "0:"),
		];
		for cc in cases {
			let vec = BenObject::String(cc.0).bencode().unwrap();
			assert_eq!(vec.len(), cc.1);
			assert_eq!(vec, cc.2.as_bytes().to_vec());
		}
		let mut string = String::new();
		for c in 'a'..'z' {
			string.push(c);
		}
		let vec = BenObject::String(string).bencode().unwrap();
		assert_eq!(vec.len(), 28);
		assert_eq!(vec, "25:abcdefghijklmnopqrstuvwxy".as_bytes().to_vec());
	}

	#[test]
	fn test_bencode_list() {
		let vec = benobject!([0, "spam"]).bencode().unwrap();
		// let vec = BenObject::List(vec![
		// 	BenObject::Int(0),
		// 	BenObject::String("spam".to_string()),
		// ])
		// .bencode()
		// .unwrap();
		assert_eq!(vec, "li0e4:spame".as_bytes().to_vec())
	}
	#[test]
	fn test_bencode_dict() {
		let vec = benobject!({
			("cow", {
				("moo", 4)
			}),
			("spam", "eggs"),
		})
		.bencode()
		.unwrap();
		// let vec = BenObject::Dict(HashMap::from_iter(
		// 	vec![
		// 		(
		// 			"cow".to_string(),
		// 			BenObject::Dict(HashMap::from_iter(
		// 				vec![("moo".to_string(), BenObject::Int(4))].into_iter(),
		// 			)),
		// 		),
		// 		("spam".to_string(), BenObject::String("eggs".to_string())),
		// 	]
		// 	.into_iter(),
		// ))
		// .bencode()
		// .unwrap();
		assert_eq!(vec, "d3:cowd3:mooi4ee4:spam4:eggse".as_bytes().to_vec());
	}
}
