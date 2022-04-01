use itertools::Itertools;
use std::collections::HashMap;
use std::convert::From;
use std::fmt;

mod macros;

mod bencode;
mod byte_buffer;
mod error;
mod parser;
pub use crate::byte_buffer::ByteBuffer;
pub use crate::error::BencodeError;

const DICT_PREFIX: u8 = b'd';
const DICT_POSTFIX: u8 = b'e';
const LIST_PREFIX: u8 = b'l';
const LIST_POSTFIX: u8 = b'e';
const INT_PREFIX: u8 = b'i';
const INT_POSTFIX: u8 = b'e';
const STR_DELIMITER: u8 = b':';
const ZERO: u8 = b'0';
const MINUS: u8 = b'-';

pub type Dict = HashMap<String, BenObject>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BenObject {
    String(String),
    Int(i64),
    List(Vec<BenObject>),
    Dict(Dict),
}

impl From<u8> for BenObject {
    fn from(val: u8) -> BenObject {
        BenObject::Int(i64::from(val))
    }
}

impl From<u16> for BenObject {
    fn from(val: u16) -> BenObject {
        BenObject::Int(i64::from(val))
    }
}

impl From<u32> for BenObject {
    fn from(val: u32) -> BenObject {
        BenObject::Int(i64::from(val))
    }
}

impl From<i8> for BenObject {
    fn from(val: i8) -> BenObject {
        BenObject::Int(i64::from(val))
    }
}

impl From<i16> for BenObject {
    fn from(val: i16) -> BenObject {
        BenObject::Int(i64::from(val))
    }
}

impl From<i32> for BenObject {
    fn from(val: i32) -> BenObject {
        BenObject::Int(i64::from(val))
    }
}

impl From<i64> for BenObject {
    fn from(val: i64) -> BenObject {
        BenObject::Int(val)
    }
}

impl<'a> From<&'a str> for BenObject {
    fn from(val: &'a str) -> BenObject {
        BenObject::String(val.to_owned())
    }
}

impl From<String> for BenObject {
    fn from(val: String) -> BenObject {
        BenObject::String(val)
    }
}

impl fmt::Display for BenObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BenObject::String(ref string) => write!(f, r#""{}""#, string),
            BenObject::Int(ref int) => write!(f, "{}", int),
            BenObject::List(ref list) => write!(f, "[{}]", itertools::join(list, ", ")),
            BenObject::Dict(ref dict) => write!(
                f,
                "{{ {} }}",
                dict.iter()
                    .sorted_by_key(|&(key, _)| key)
                    .format_with(", ", |(k, v), f| f(&format_args!(
                        r#"("{}", {})"#,
                        // 转成 char 才能打印出原始的key(不转的话是字节)
                        k.as_bytes().to_vec().iter().map(|b| *b as char).format(""),
                        v
                    )))
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_display_string() {
        assert_eq!(BenObject::String("".to_string()).to_string(), r#""""#);
    }

    #[test]
    fn test_display_int() {
        assert_eq!(BenObject::Int(0).to_string(), "0");
    }

    #[test]
    fn test_display_list() {
        assert_eq!(benobject!([0, "spam"]).to_string(), r#"[0, "spam"]"#);
    }

    #[test]
    fn test_display_dict() {
        assert_eq!(
            BenObject::Dict(HashMap::from_iter(
                vec![
                    (
                        "cow".to_string(),
                        BenObject::Dict(HashMap::from_iter(
                            vec![("moo".to_string(), BenObject::Int(4))].into_iter()
                        ))
                    ),
                    ("spam".to_string(), BenObject::String("eggs".to_string()))
                ]
                .into_iter()
            ))
            .to_string(),
            r#"{ ("cow", { ("moo", 4) }), ("spam", "eggs") }"#,
        );
    }
}
