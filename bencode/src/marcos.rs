use super::*;

#[macro_export]
macro_rules! bencode {
    ([ $( $element:tt ),* ]) => {
        $crate::BenObject::List(vec![ $( bencode!($element) ),* ])
    };
    ([ $( $element:tt ),+ ,]) => {
        bencode!([ $( $element ),* ])
    };
    ({ $( ($key:tt, $val:tt) ),* }) => {
        $crate::BenObject::Dict(
            ::std::collections::HashMap::from_iter(
                vec![ $( ($key.to_owned(), bencode!($val)) ),* ].into_iter()
            )
        )
    };
    ({ $( ($key:tt, $val:tt) ),+ ,}) => {
        bencode!({ $( ($key, $val) ),* })
    };
    (r{ $( ( [ $( $key:tt ),+ ,] , $val:tt) ),* }) => {
        bencode!(r{ $( ( [ $( $key ),* ], $val) ),* })
    };
    (r{ $( ( [ $( $key:tt ),* ] , $val:tt) ),+ ,}) => {
        bencode!(r{ $( ( [ $( $key ),* ], $val) ),* })
    };
    ($other:expr) => {
        $crate::BenObject::from($other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_u8_to_int() {
        assert_eq!(bencode!(0_u8), BenObject::Int(0))
    }

    #[test]
    fn test_u16_to_int() {
        assert_eq!(bencode!(0_u16), BenObject::Int(0))
    }

    #[test]
    fn test_u32_to_int() {
        assert_eq!(bencode!(0_u32), BenObject::Int(0))
    }

    #[test]
    fn test_i8_to_int() {
        assert_eq!(bencode!(0_i8), BenObject::Int(0))
    }

    #[test]
    fn test_i16_to_int() {
        assert_eq!(bencode!(0_i16), BenObject::Int(0))
    }

    #[test]
    fn test_i32_to_int() {
        assert_eq!(bencode!(0_i32), BenObject::Int(0))
    }

    #[test]
    fn test_i64_to_int() {
        assert_eq!(bencode!(0_i64), BenObject::Int(0))
    }

    #[test]
    fn test_str_ref_to_int() {
        assert_eq!(bencode!(""), BenObject::String("".to_owned()))
    }

    #[test]
    fn test_string_to_int() {
        let string = "".to_owned();
        assert_eq!(bencode!(string), BenObject::String("".to_owned()))
    }

    #[test]
    fn test_list() {
        assert_eq!(
            bencode!([0x01, "0x02", [0x03]]),
            BenObject::List(vec![
                BenObject::Int(0x01),
                BenObject::String("0x02".to_owned()),
                BenObject::List(vec![BenObject::Int(0x03)]),
            ])
        )
    }

    #[test]
    fn test_list_empty() {
        assert_eq!(bencode!([]), BenObject::List(vec![]))
    }

    #[test]
    fn test_dict() {
        assert_eq!(
            bencode!({ ("cow", { ("moo", 4) }), ("spam", "eggs") }),
            BenObject::Dict(HashMap::from_iter(
                vec![
                    (
                        "cow".to_owned(),
                        BenObject::Dict(HashMap::from_iter(
                            vec![("moo".to_owned(), BenObject::Int(4_i64))].into_iter(),
                        )),
                    ),
                    ("spam".to_owned(), BenObject::String("eggs".to_owned())),
                ]
                .into_iter(),
            ))
        )
    }

    #[test]
    fn test_dict_empty() {
        assert_eq!(bencode!({}), BenObject::Dict(HashMap::new()))
    }
}
