use super::*;

impl BObject {
    // pub fn parse(data: &mut ByteBuffer) -> Result<BObject, BencodeError> {

    // }
}

// ANCHOR: decoder

// fn decode_int(r: &mut ByteBuffer, delimiter: u8) -> Result<BObject> {
//     if let Ok(b) = r.read_bytes(1) {
//         if b != b"i" {
//             // TODO: define error with thiserr
//             return Err("expect char i");
//         }
//     }
//     decode_decimal(r);
//     if let Ok(b) = r.read_bytes(1) {
//         if b != b"e" {
//             // TODO: define error with thiserr
//             return Err("expect char e");
//         }
//     }
// }

fn decode_decimal(r: &mut ByteBuffer) -> (i64, i64) {
    let mut sign = 1;
    let mut val = 0;
    let mut len = 0;
    let mut byte: &u8 = &0;
    if let Some(b) = r.next() {
        len += 1;
        if b == &MINUS {
            sign = -1;
            if let Some(b) = r.next() {
                byte = b;
                len += 1;
            }
        }
    }

    loop {
        if !check_num(*byte) {
            r.push_back(1);
            len -= 1;
            return (sign * val, len);
        }
        val = (val * 10) + (byte - ZERO_ASCII) as i64;

        if let Some(b) = r.next() {
            byte = b;
            len += 1;
        }
    }
}
// ANCHOR_END: decoder

fn check_num(num: u8) -> bool {
    num >= ZERO_ASCII && num <= NINE_ASCII
}
