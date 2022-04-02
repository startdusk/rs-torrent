#[macro_export]
macro_rules! benobject {
    ([ $( $element:tt ),* ]) => {
        $crate::BenObject::List(vec![ $( benobject!($element) ),* ])
    };
    ([ $( $element:tt ),+ ,]) => {
        benobject!([ $( $element ),* ])
    };
    (( $( $element:tt ),+ ,)) => {
        benobject!(( $( $element ),* ))
    };
    ({ $( ($key:tt, $val:tt) ),* }) => {
        $crate::BenObject::Dict(
            ::std::collections::HashMap::from_iter(
                vec![ $( ($key.to_owned(), benobject!($val)) ),* ].into_iter()
            )
        )
    };
    ({ $( ($key:tt, $val:tt) ),+, }) => {
        $crate::BenObject::Dict(
            ::std::collections::HashMap::from_iter(
                vec![ $( ($key.to_owned(), benobject!($val)) ),* ].into_iter()
            )
        )
    };
    ({ $( ($key:tt, $val:tt) ),+ ,}) => {
        benobject!({ $( ($key, $val) ),* })
    };

    ({ $( ($key:tt, $val:tt) ),* ,}) => {
        benobject!({ $( ($key, $val) ),* })
    };
    (r{ $( ( [ $( $key:tt ),+ ,] , $val:tt) ),* }) => {
        benobject!(r{ $( ( [ $( $key ),* ], $val) ),* })
    };
    (r{ $( ( [ $( $key:tt ),* ] , $val:tt) ),+ ,}) => {
        benobject!(r{ $( ( [ $( $key ),* ], $val) ),* })
    };
    ($other:expr) => {
        $crate::BenObject::from($other)
    }
}

#[cfg(test)]
mod tests {
    use crate::BenObject;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    #[test]
    fn test_u8_to_int() {
        assert_eq!(benobject!(0_u8), BenObject::Int(0))
    }

    #[test]
    fn test_u16_to_int() {
        assert_eq!(benobject!(0_u16), BenObject::Int(0))
    }

    #[test]
    fn test_u32_to_int() {
        assert_eq!(benobject!(0_u32), BenObject::Int(0))
    }

    #[test]
    fn test_i8_to_int() {
        assert_eq!(benobject!(0_i8), BenObject::Int(0))
    }

    #[test]
    fn test_i16_to_int() {
        assert_eq!(benobject!(0_i16), BenObject::Int(0))
    }

    #[test]
    fn test_i32_to_int() {
        assert_eq!(benobject!(0_i32), BenObject::Int(0))
    }

    #[test]
    fn test_i64_to_int() {
        assert_eq!(benobject!(0_i64), BenObject::Int(0))
    }

    #[test]
    fn test_str_ref_to_int() {
        assert_eq!(benobject!(""), BenObject::String("".to_owned()))
    }

    #[test]
    fn test_string_to_int() {
        let string = "".to_owned();
        assert_eq!(benobject!(string), BenObject::String("".to_owned()))
    }

    #[test]
    fn test_list() {
        assert_eq!(
            benobject!([0x01, "0x02", [0x03]]),
            BenObject::List(vec![
                BenObject::Int(0x01),
                BenObject::String("0x02".to_owned()),
                BenObject::List(vec![BenObject::Int(0x03)]),
            ])
        )
    }

    #[test]
    fn test_list_empty() {
        assert_eq!(benobject!([]), BenObject::List(vec![]))
    }

    #[test]
    fn test_dict() {
        assert_eq!(
            benobject!({ ("cow", { ("moo", 4), ("b00", 6) }), ("spam", "eggs") }),
            BenObject::Dict(HashMap::from_iter(
                vec![
                    (
                        "cow".to_owned(),
                        BenObject::Dict(HashMap::from_iter(
                            vec![
                                ("moo".to_owned(), BenObject::Int(4_i64)),
                                ("b00".to_owned(), BenObject::Int(6))
                            ]
                            .into_iter(),
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
        assert_eq!(benobject!({}), BenObject::Dict(HashMap::new()))
    }
}
