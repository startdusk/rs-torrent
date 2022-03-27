use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i64),
    List(Vec<BObject>),
    Obj(HashMap<String, BObject>),
}

#[derive(Debug, Clone)]
pub struct BObject {
    val: Value,
}

impl BObject {
    pub fn bencode(self, w: &mut bytebuffer::ByteBuffer) -> usize {
        let mut wlen = 0;
        match self.val {
            Value::Int(num) => wlen += encode_int(w, num),
            Value::Str(s) => wlen += encode_str(w, s.as_str()),
            Value::List(list) => {
                w.write_bytes(b"l");
                wlen += 1;
                for elem in list {
                    wlen += elem.bencode(w);
                }
                w.write_bytes(b"e");
                wlen += 1;
            }
            Value::Obj(dict) => {
                w.write_bytes(b"d");
                wlen += 1;
                for (k, v) in dict.into_iter() {
                    wlen += encode_str(w, k.as_str());
                    wlen += v.bencode(w);
                }
                w.write_bytes(b"e");
                wlen += 1;
            }
        }

        wlen
    }
}

// 字节串的格式为 字节串长度:内容，其中 字节串长度 是 ASCII 编码格式的整数字符串，单位为字节
// 4:abcd 表示4个字节的串 "abcd"
// 0:     表示0个字节的串 ""
fn encode_str(w: &mut bytebuffer::ByteBuffer, s: &str) -> usize {
    let slen = s.len();
    let mut wlen = encode_decimal(w, slen as i64);
    w.write_bytes(b":");
    wlen += 1;
    w.write_string(s);
    wlen += slen;
    if let Err(_) = w.flush() {
        return 0;
    }
    wlen
}

// 整数的格式为 i整数e，其中 整数 是 ASCII 编码格式的整数字符串
// i1234e 表示整数 1234
// 注意：
//      1.i-0e 是无效编码
//      2.除了 i0e 之外，一切以0开头的整数如 i03e, i011e 都是无效的编码
//      3.虽然并未规定整数类型的最大值，但是 64位 整数的支持是强制的、必不可少的，以支持超过 4GB 大小的文件
fn encode_int(w: &mut bytebuffer::ByteBuffer, val: i64) -> usize {
    let mut wlen = 0;
    w.write_bytes(b"i");
    wlen += 1;
    let nlen = encode_decimal(w, val);
    wlen += nlen;
    w.write_bytes(b"e");
    wlen += 1;

    if let Err(_) = w.flush() {
        return 0;
    }
    wlen
}

fn encode_decimal(w: &mut bytebuffer::ByteBuffer, val: i64) -> usize {
    let mut val = val;
    let mut len = 0;
    if val == 0 {
        w.write_bytes(b"0");
        len += 1;
        return len;
    }

    if val < 0 {
        w.write_bytes(b"-");
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
        let v = format!("0{}", (val / dividend));
        w.write_bytes(v.as_bytes());
        len += 1;
        if dividend == 1 {
            return len;
        }
        val %= dividend;
        dividend /= 10;
    }
}

// decoder start
fn decode_int(r: bytebuffer::ByteBuffer) -> Result<i64, Err> {
    if let Ok(b) = r.read_bytes(1) {
        if b != b"i" {
            // TODO: define error with thiserr
            return Err("expect char i");
        }
    }
    decode_decimal(r);
    if let Ok(b) = r.read_bytes(1) {
        if b != b"e" {
            // TODO: define error with thiserr
            return Err("expect char e");
        }
    }
}

fn decode_decimal(r: bytebuffer::ByteBuffer) -> (i64, i64) {
    let mut sign = 1;
    let mut len = 0;
    if let Ok(b) = r.read_bytes(1) {
        if b == b"-" {
            sign = -1;
            // TODO: push back
        }
    }

    (sign, len)
}
// decoder end

fn check_num(num: char) -> bool {
    num >= '0' && num <= '9'
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decimal() {
        let y = 1 / 10_i64;
        let y_bytes = y.to_be_bytes();
        let original_y = i64::from_ne_bytes(y_bytes); // original_y = 255.255 = y
        println!("{:?}", y.to_ne_bytes());
    }
}
