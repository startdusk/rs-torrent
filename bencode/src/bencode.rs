use super::*;

use std::collections::HashMap;
use std::hash::BuildHasher;
use std::io::Write;

impl BObject {
    pub fn bencode<W>(&self, w: &mut W) -> usize
    where
        W: Write,
    {
        let mut wlen = 0;
        match *self {
            BObject::Int(num) => wlen += write_int(w, num),
            BObject::Str(ref s) => wlen += write_string(w, s),
            BObject::List(ref list) => wlen += write_list(w, list),
            BObject::Dict(ref dict) => wlen += write_dict(w, dict),
        }

        wlen
    }
}

// ANCHOR: encoder
// 列表的格式为 l不限数量个BE量e（小写L开头）
// l4:spam4:eggse 表示 [ "spam", "eggs" ]
// li123e5:helloi111ee 表示 [ 123, "hello", 111 ]
// le 表示 [] 空列表
fn write_list<W, L>(w: &mut W, list: L) -> usize
where
    W: Write,
    L: AsRef<[BObject]>,
{
    let mut wlen = 0;
    let list = list.as_ref();
    w.write_all(&[LIST_PREFIX]).ok();
    wlen += 1;
    for item in list {
        wlen += item.bencode(w);
    }
    w.write_all(&[LIST_POSTFIX]).ok();
    wlen += 1;
    wlen
}

// 字典的格式为 d不限数量个字段e(小写D开头)
// 字段 是指一种 key-value 结构，其中 key 是一个BE字节串，一个字段的格式为 一个BE字节串+一个BE量
// d3:cow3:moo4:spam4:eggse 表示 {"cow":"moo", "spam": "egge"}
// d4:name5:Angus3:agei23ee 表示 {"name":"Angus", "age":23}
// d4:spaml1:a1:bee 表示 {"spam":["a","b"]}
// de 表示 {}
// 注意：key 是 BE字节串，而不是字符串，因此 key 的比较是二进制比较而不是字符串比较
fn write_dict<W, S>(w: &mut W, dict: &HashMap<String, BObject, S>) -> usize
where
    W: Write,
    S: BuildHasher,
{
    let mut sorted = dict.iter().collect::<Vec<(&String, &BObject)>>();
    sorted.sort_by_key(|&(key, _)| key.as_bytes());
    let mut wlen = 0;
    w.write_all(&[DICT_PREFIX]).ok();
    wlen += 1;
    for (key, val) in sorted {
        wlen += write_string(w, key);
        wlen += val.bencode(w);
    }
    w.write_all(&[DICT_POSTFIX]).ok();
    wlen += 1;
    wlen
}

// 字节串的格式为 字节串长度:内容，其中 字节串长度 是 ASCII 编码格式的整数字符串，单位为字节
// 4:abcd 表示4个字节的串 "abcd"
// 0:     表示0个字节的串 ""
fn write_string<W, S>(w: &mut W, s: S) -> usize
where
    W: Write,
    S: AsRef<str>,
{
    let s = s.as_ref();
    let slen = s.len();
    let mut wlen = write_decimal(w, slen as i64);
    w.write_all(&[STR_DELIMITER]).ok();
    wlen += 1;
    w.write_all(s.as_bytes()).ok();
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
fn write_int<W>(w: &mut W, val: i64) -> usize
where
    W: Write,
{
    let mut wlen = 0;
    w.write_all(&[INT_PREFIX]).ok();
    wlen += 1;
    let nlen = write_decimal(w, val);
    wlen += nlen;
    w.write_all(&[INT_POSTFIX]).ok();
    wlen += 1;

    if let Err(_) = w.flush() {
        return 0;
    }
    wlen
}

fn write_decimal<W>(w: &mut W, val: i64) -> usize
where
    W: Write,
{
    let mut val = val;
    let mut len = 0;
    if val == 0 {
        w.write_all(&[ZERO]).ok();
        len += 1;
        return len;
    }

    if val < 0 {
        w.write_all(&[MINUS]).ok();
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
        w.write_all(v.as_bytes()).ok();
        len += 1;
        if dividend == 1 {
            return len;
        }
        val %= dividend;
        dividend /= 10;
    }
}
// ANCHOR_END: encoder
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
