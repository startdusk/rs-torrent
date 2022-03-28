use super::*;

impl BenObject {
    fn parse(r: &mut ByteBuffer) -> Result<BenObject, BencodeError> {
        match Self::peek_byte(r)? {
            DICT_PREFIX => {
                r.advance(1);
                let mut dict = HashMap::new();
                loop {
                    match r.peek() {
                        Some(&b) => {
                            if b == LIST_POSTFIX {
                                r.advance(1);
                                return Ok(BenObject::Dict(dict));
                            }
                        }
                        None => return Err(BencodeError::EOF),
                    }

                    let mut key;
                    match read_string(r) {
                        Ok(obj) => {
                            if let BenObject::Str(k) = obj {
                                key = k
                            }
                        }
                        Err(e) => return Err(e),
                    };

                    let mut val;
                    match Self::parse(r) {
                        Ok(obj) => val = obj,
                        Err(e) => return Err(e),
                    }
                    // dict.insert(key, val);
                }
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
    let (num, len) = read_decimal(r);
    if len == 0 {
        return Err(BencodeError::ExpectNumberError);
    }

    let mut string = String::with_capacity(num as usize);
    for _ in 0..len {
        if let Some(b) = r.next() {
            string.push(char::from(*b));
        }
    }

    Ok(BenObject::Str(string))
}

fn read_int(r: &mut ByteBuffer) -> Result<BenObject, BencodeError> {
    match r.next() {
        Some(b) => {
            if b != &INT_PREFIX {
                return Err(BencodeError::ExpectCharIError(*b));
            }
        }
        None => return Err(BencodeError::EOF),
    }
    let (val, _) = read_decimal(r);
    match r.next() {
        Some(b) => {
            if b != &INT_POSTFIX {
                return Err(BencodeError::ExpectCharEError(*b));
            }
        }
        None => return Err(BencodeError::EOF),
    }

    Ok(BenObject::Int(val))
}

fn read_decimal(r: &mut ByteBuffer) -> (i64, i64) {
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
