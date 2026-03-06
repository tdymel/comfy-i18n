use std::collections::HashMap;

use crate::{identifier::Identifier, template::Template};

#[derive(Debug, Clone)]
pub enum CompositeValue {
    Struct,
    Tuple,
    List { amount: usize }, // Also contains ByteString as a list of u8
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    String(StringValue),
    Char(char),
    Float(FloatValue),
    Integer(IntegerValue),
    Bool(bool),
    Cast { expression: String, ty: String },
}

#[derive(Debug, Clone)]
pub enum NodeValue<AstValue> {
    Composite {
        children: HashMap<Identifier, AstValue>,
        value: CompositeValue,
    },
    Literal(LiteralValue),
}

#[derive(Debug, Clone)]
pub enum FloatValue {
    F64(f64),
    F32(f32),
}

#[derive(Debug, Clone)]
pub enum IntegerValue {
    I128(i128),
    U128(u128),
    I64(i64),
    U64(u64),
    I32(i32),
    U32(u32),
    I16(i16),
    U16(u16),
    I8(i8),
    U8(u8), // Byte
    Usize(usize),
}

impl IntegerValue {
    pub fn to_usize(&self) -> Option<usize> {
        match *self {
            IntegerValue::I128(v) => Self::convert_signed(v),
            IntegerValue::U128(v) => Self::convert_unsigned(v),
            IntegerValue::I64(v) => Self::convert_signed(v),
            IntegerValue::U64(v) => Self::convert_unsigned(v),
            IntegerValue::I32(v) => Self::convert_signed(v),
            IntegerValue::U32(v) => Some(v as usize),
            IntegerValue::I16(v) => Self::convert_signed(v),
            IntegerValue::U16(v) => Some(v as usize),
            IntegerValue::I8(v) => Self::convert_signed(v),
            IntegerValue::U8(v) => Some(v as usize),
            IntegerValue::Usize(v) => Some(v),
        }
    }

    fn convert_signed<T: Into<i128> + PartialOrd>(value: T) -> Option<usize> {
        let value: i128 = value.into();
        if value >= 0 && value <= usize::MAX as i128 {
            Some(value as usize)
        } else {
            None
        }
    }

    fn convert_unsigned<T: Into<u128> + PartialOrd>(value: T) -> Option<usize> {
        let value: u128 = value.into();
        if value <= usize::MAX as u128 {
            Some(value as usize)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum StringValue {
    Literal(String),
    Template(Template),
}
