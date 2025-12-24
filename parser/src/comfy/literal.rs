use std::collections::HashMap;

use comfy_i18n_ast::{
    CompositeValue, FloatValue, FormatArg, FormatPart, Identifier, IntegerValue, LiteralValue,
    NodeValue, SpannedAst, StringValue,
};
use proc_macro2::Span;
use quote::ToTokens;
use regex::Regex;
use syn::{
    Error, Expr,
    parse::{Parse, ParseStream},
};

use crate::comfy::field::Fields;

pub struct Literal(pub NodeValue<SpannedAst<Span>>);

impl Parse for Literal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<Expr>()? {
            Expr::Lit(syn::ExprLit { lit, .. }) => match lit {
                syn::Lit::Int(lit_int) => Ok(Literal(NodeValue::Literal(LiteralValue::Integer(
                    match lit_int.suffix() {
                        "i128" => IntegerValue::I128(lit_int.base10_parse()?),
                        "u128" => IntegerValue::U128(lit_int.base10_parse()?),
                        "i64" => IntegerValue::I64(lit_int.base10_parse()?),
                        "u64" => IntegerValue::U64(lit_int.base10_parse()?),
                        "u32" => IntegerValue::U32(lit_int.base10_parse()?),
                        "i16" => IntegerValue::I16(lit_int.base10_parse()?),
                        "u16" => IntegerValue::U16(lit_int.base10_parse()?),
                        "i8" => IntegerValue::I8(lit_int.base10_parse()?),
                        "u8" => IntegerValue::U8(lit_int.base10_parse()?),
                        "i32" | _ => IntegerValue::I32(lit_int.base10_parse()?),
                    },
                )))),
                syn::Lit::Byte(lit_byte) => Ok(Literal(NodeValue::Literal(LiteralValue::Integer(
                    IntegerValue::U8(lit_byte.value()),
                )))),
                syn::Lit::Float(lit_float) => Ok(Literal(NodeValue::Literal(LiteralValue::Float(
                    match lit_float.suffix() {
                        "f64" => FloatValue::F64(lit_float.base10_parse()?),
                        "f32" | _ => FloatValue::F32(lit_float.base10_parse()?),
                    },
                )))),
                syn::Lit::Bool(lit_bool) => Ok(Literal(NodeValue::Literal(LiteralValue::Bool(
                    lit_bool.value(),
                )))),
                syn::Lit::Char(lit_char) => Ok(Literal(NodeValue::Literal(LiteralValue::Char(
                    lit_char.value(),
                )))),
                syn::Lit::Str(lit_str) => Ok(Literal(NodeValue::Literal(LiteralValue::String({
                    // Not the best regex, but it gets the job done!
                    let re = Regex::new(r"([^{}]+)*(\{(((:{2})?[^{}:]+)*)(:[^{}]+)?\})?").unwrap();
                    let mut args = Vec::new();

                    for cap in re.captures_iter(&lit_str.value()) {
                        if let Some(literal_part) = cap.get(1) {
                            args.push(FormatPart::Literal(literal_part.as_str().to_string()));
                        }

                        if let Some(arg_part) = cap.get(3) {
                            let name = arg_part.as_str().to_string();
                            let suffix = cap
                                .get(6)
                                .map(|it| it.as_str())
                                .map(|it| (&it[1..]).to_string());
                            args.push(FormatPart::Arg(FormatArg { name, suffix }));
                        }
                    }

                    if args.iter().any(|part| matches!(part, FormatPart::Arg(..))) {
                        StringValue::Format(args)
                    } else {
                        StringValue::Literal(lit_str.value())
                    }
                })))),
                syn::Lit::ByteStr(lit_byte) => Ok(Literal(NodeValue::Composite {
                    children: HashMap::from(Fields(
                        lit_byte
                            .value()
                            .into_iter()
                            .enumerate()
                            .map(|(index, byte)| {
                                SpannedAst::new(
                                    Identifier::Element(index),
                                    lit_byte.span(),
                                    NodeValue::Literal(LiteralValue::Integer(IntegerValue::U8(
                                        byte,
                                    ))),
                                )
                            })
                            .collect::<Vec<_>>(),
                    )),
                    value: CompositeValue::List {
                        amount: lit_byte.value().len(),
                    },
                })),
                _ => Err(Error::new(
                    input.span(),
                    "comfy-i18n-parser: Unsupported literal",
                )),
            },
            Expr::Cast(cast_expr) => Ok(Literal(NodeValue::Literal(LiteralValue::Cast {
                expression: cast_expr.to_token_stream().to_string(),
            }))),
            _ => Err(Error::new(
                input.span(),
                "comfy-i18n-parser: Expected literal",
            )),
        }
    }
}
