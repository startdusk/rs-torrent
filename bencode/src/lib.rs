use std::collections::HashMap;

pub mod bencode;
pub mod byte_buffer;
pub mod error;
pub mod parser;
pub use crate::byte_buffer::ByteBuffer;
pub use crate::error::BencodeError;

const DICT_PREFIX: u8 = b'd';
const DICT_POSTFIX: u8 = b'e';
const LIST_PREFIX: u8 = b'l';
const LIST_POSTFIX: u8 = b'e';
const INT_PREFIX: u8 = b'i';
const INT_POSTFIX: u8 = b'e';
const STR_DELIMITER: u8 = b':';
const ZERO: u8 = 0;
const MINUS: u8 = b'-';
const ZERO_ASCII: u8 = 48; // '0'
const NINE_ASCII: u8 = 57; // '9'

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BenObject {
    Str(String),
    Int(i64),
    List(Vec<BenObject>),
    Dict(HashMap<String, BenObject>),
}
