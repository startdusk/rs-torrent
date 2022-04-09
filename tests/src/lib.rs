#[cfg(test)]
mod tests {
    use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};
    #[test]
    fn it_works() {
        /// Contains the characters that need to be URL encoded according to:
        /// https://en.wikipedia.org/wiki/Percent-encoding#Types_of_URI_characters
        const URL_ENCODE_RESERVED: &AsciiSet = &NON_ALPHANUMERIC
            .remove(b'-')
            .remove(b'_')
            .remove(b'~')
            .remove(b'.');
        let v = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];

        let bytes = hex::encode_upper(v);

        dbg!(bytes);

        let hash: Vec<u8> = vec![
            177, 17, 129, 60, 230, 15, 66, 145, 151, 52, 130, 61, 245, 236, 32, 189, 30, 4, 231,
            247,
        ];

        let info_hash = percent_encoding::percent_encode(&hash, URL_ENCODE_RESERVED);
        let s = format!("{}", info_hash);
        dbg!(s);
    }
}
