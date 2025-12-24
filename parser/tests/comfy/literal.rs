use comfy_i18n_ast::{
    CompositeValue, FloatValue, IntegerValue, LiteralValue, NodeValue, SpannedAst, StringValue,
};
use comfy_i18n_parser::Parser;
use paste::paste;

#[test]
fn string() {
    let result = "\"Some string\"".parse_literal().expect("Parsing failed");
    if let NodeValue::Literal(LiteralValue::String(StringValue::Literal(str))) = result {
        assert_eq!(str, "Some string")
    } else {
        panic!();
    }
}

#[test]
fn format() {
    let result = "\"This is a {formatted} string with {crate::abc:some_mod} multiple {self.fn():dsfs} args. {crate::abc.dsfs():bla} {self.abc}\"".parse_literal().expect("Parsing failed");
    assert_eq!(
        format!("{:?}", result),
        "Literal(String(Format([Literal(\"This is a \"), Arg(FormatArg { name: \"formatted\", suffix: None }), Literal(\" string with \"), Arg(FormatArg { name: \"crate::abc\", suffix: Some(\"some_mod\") }), Literal(\" multiple \"), Arg(FormatArg { name: \"self.fn()\", suffix: Some(\"dsfs\") }), Literal(\" args. \"), Arg(FormatArg { name: \"crate::abc.dsfs()\", suffix: Some(\"bla\") }), Literal(\" \"), Arg(FormatArg { name: \"self.abc\", suffix: None })])))"
    );
}

macro_rules! test_integer {
    ($type:ty, $variant:ident) => {
        paste! {
            #[test]
            fn [<integer_ $variant:lower>]() {
                let input = format!("42{}", stringify!($variant)).to_lowercase();
                let expected: $type = 42;
                let result = input.parse_literal().expect("Parsing failed");

                if let NodeValue::Literal(LiteralValue::Integer(IntegerValue::$variant(integer))) = result {
                    assert_eq!(integer, expected);
                } else {
                    panic!();
                }
            }
        }
    };
}

test_integer!(i128, I128);
test_integer!(u128, U128);
test_integer!(i64, I64);
test_integer!(u64, U64);
test_integer!(i32, I32);
test_integer!(u32, U32);
test_integer!(i16, I16);
test_integer!(u16, U16);
test_integer!(i8, I8);
test_integer!(u8, U8);

#[test]
fn integer_default() {
    let result = "450".parse_literal().expect("Parsing failed");
    if let NodeValue::Literal(LiteralValue::Integer(IntegerValue::I32(integer))) = result {
        assert_eq!(integer, 450i32);
    } else {
        panic!();
    }
}

macro_rules! test_float {
    ($type:ty, $variant:ident) => {
        paste! {
            #[test]
            fn [<float_ $variant:lower>]() {
                let input = format!("42.3{}", stringify!($variant)).to_lowercase();
                let expected: $type = 42.3;
                let result = input.parse_literal().expect("Parsing failed");

                if let NodeValue::Literal(LiteralValue::Float(FloatValue::$variant(float))) = result {
                    assert_eq!(float, expected);
                } else {
                    panic!();
                }
            }
        }
    };
}

test_float!(f64, F64);
test_float!(f32, F32);

#[test]
fn float_default() {
    let result = "42.3".parse_literal().expect("Parsing failed");
    if let NodeValue::Literal(LiteralValue::Float(FloatValue::F32(float))) = result {
        assert_eq!(float, 42.3f32);
    } else {
        panic!();
    }
}

#[test]
fn char() {
    let result = "'C'".parse_literal().expect("Parsing failed");
    if let NodeValue::Literal(LiteralValue::Char(char)) = result {
        assert_eq!(char, 'C')
    } else {
        panic!();
    }
}

#[test]
fn bool() {
    let result = "true".parse_literal().expect("Parsing failed");
    if let NodeValue::Literal(LiteralValue::Bool(bool)) = result {
        assert_eq!(bool, true)
    } else {
        panic!();
    }
}

#[test]
fn byte() {
    let result = "b'A'".parse_literal().expect("Parsing failed");
    if let NodeValue::Literal(LiteralValue::Integer(IntegerValue::U8(byte))) = result {
        assert_eq!(byte, b'A')
    } else {
        panic!();
    }
}

#[test]
fn byte_str() {
    let result: NodeValue<SpannedAst<proc_macro2::Span>> =
        "b\"Hello\"".parse_literal().expect("Parsing failed");
    if let NodeValue::Composite {
        children,
        value: CompositeValue::List { amount },
    } = result
    {
        let mut child_nodes = children.values().collect::<Vec<_>>();
        child_nodes.sort_by(|l, r| l.identifier.cmp(&r.identifier));

        let byte_arr = child_nodes
            .into_iter()
            .map(|child| {
                if let NodeValue::Literal(LiteralValue::Integer(IntegerValue::U8(val))) =
                    child.value
                {
                    val
                } else {
                    panic!()
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(amount, 5);
        assert_eq!(&byte_arr, b"Hello");
    } else {
        panic!();
    }
}
