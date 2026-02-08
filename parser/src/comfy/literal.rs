use std::collections::HashMap;

use comfy_i18n_ast::{
    ArgumentKey, ArgumentName, AstRefOrigin, CompositeValue, FloatValue, Identifier, IntegerValue,
    LiteralValue, NameRef, NodeValue, Piece, SpannedAst, Specifier, StringValue, Template,
};
use dfmt::{AlternateForm, PadZero, Sign};
use proc_macro2::Span;
use quote::ToTokens;
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
                        _ => IntegerValue::I32(lit_int.base10_parse()?),
                    },
                )))),
                syn::Lit::Byte(lit_byte) => Ok(Literal(NodeValue::Literal(LiteralValue::Integer(
                    IntegerValue::U8(lit_byte.value()),
                )))),
                syn::Lit::Float(lit_float) => Ok(Literal(NodeValue::Literal(LiteralValue::Float(
                    match lit_float.suffix() {
                        "f64" => FloatValue::F64(lit_float.base10_parse()?),
                        _ => FloatValue::F32(lit_float.base10_parse()?),
                    },
                )))),
                syn::Lit::Bool(lit_bool) => Ok(Literal(NodeValue::Literal(LiteralValue::Bool(
                    lit_bool.value(),
                )))),
                syn::Lit::Char(lit_char) => Ok(Literal(NodeValue::Literal(LiteralValue::Char(
                    lit_char.value(),
                )))),
                syn::Lit::Str(lit_str) => Ok(Literal(NodeValue::Literal(LiteralValue::String({
                    let pieces = parse_template(&lit_str.value(), lit_str.span())?;

                    if pieces
                        .iter()
                        .any(|part| matches!(part, Piece::Argument { .. }))
                    {
                        StringValue::Template(Template(pieces))
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

fn parse_template(input: &str, span: Span) -> Result<Vec<Piece>, Error> {
    let mut pieces: Vec<Piece> = Vec::with_capacity(10);

    let mut cursor = 0;
    let mut current_char = 0;
    let mut bracket = None;
    let mut separator = None;
    let mut internal_index = 0;

    let error = || Error::new(span, "comfy-i18n-parser: Template parsing failure");

    let chars = input.as_bytes();
    while current_char < chars.len() {
        let char = chars[current_char];
        match char {
            b':' if bracket == Some(b'{') => {
                separator = Some(current_char);
            }
            b'{' | b'}' => match (bracket, char) {
                (None, _) => {
                    if cursor < current_char {
                        pieces.push(Piece::Literal(input[cursor..current_char].to_string()));
                    }
                    bracket = Some(char);
                    cursor = current_char;
                }
                (Some(b'{'), b'}') => {
                    let specifier = if let Some(seperator_index) = separator {
                        Some(
                            dfmt::Specifier::parse(
                                &input[seperator_index + 1..current_char],
                                &mut internal_index,
                            )
                            .map(|spec| Specifier {
                                ty: match spec.ty {
                                    dfmt::Type::Binary => comfy_i18n_ast::Type::Binary,
                                    dfmt::Type::Octal => comfy_i18n_ast::Type::Octal,
                                    dfmt::Type::LowerHex => comfy_i18n_ast::Type::LowerHex,
                                    dfmt::Type::UpperHex => comfy_i18n_ast::Type::UpperHex,
                                    dfmt::Type::Pointer => comfy_i18n_ast::Type::Pointer,
                                    dfmt::Type::LowerExp => comfy_i18n_ast::Type::LowerExp,
                                    dfmt::Type::UpperExp => comfy_i18n_ast::Type::UpperExp,
                                    dfmt::Type::Debug => comfy_i18n_ast::Type::Debug,
                                    dfmt::Type::Display => comfy_i18n_ast::Type::Display,
                                    _ => unreachable!(),
                                },
                                alternate_form: spec.alternate_form == AlternateForm::Activated,
                                fill_character: spec.fill_character,
                                pad_zero: spec.pad_zero == PadZero::Activated,
                                sign: spec.sign == Sign::Plus,
                                alignment: match spec.alignment {
                                    dfmt::Alignment::Left => comfy_i18n_ast::Alignment::Left,
                                    dfmt::Alignment::Center => comfy_i18n_ast::Alignment::Center,
                                    dfmt::Alignment::Right => comfy_i18n_ast::Alignment::Right,
                                    dfmt::Alignment::Auto => comfy_i18n_ast::Alignment::Auto,
                                },
                                width: match spec.width {
                                    dfmt::Width::Dynamic(argument_key) => {
                                        comfy_i18n_ast::Width::Dynamic(match argument_key {
                                            dfmt::ArgumentKey::Index(index) => {
                                                ArgumentKey::Index(index)
                                            }
                                            dfmt::ArgumentKey::Name(name) => {
                                                ArgumentKey::Name(name)
                                            }
                                        })
                                    }
                                    dfmt::Width::Fixed(amount) => {
                                        comfy_i18n_ast::Width::Fixed(amount)
                                    }
                                },
                                precision: match spec.precision {
                                    dfmt::Precision::Auto => comfy_i18n_ast::Precision::Auto,
                                    dfmt::Precision::Dynamic(argument_key) => {
                                        comfy_i18n_ast::Precision::Dynamic(match argument_key {
                                            dfmt::ArgumentKey::Index(index) => {
                                                ArgumentKey::Index(index)
                                            }
                                            dfmt::ArgumentKey::Name(name) => {
                                                ArgumentKey::Name(name)
                                            }
                                        })
                                    }
                                    dfmt::Precision::Fixed(amount) => {
                                        comfy_i18n_ast::Precision::Fixed(amount)
                                    }
                                },
                            })
                            .map_err(|_| error())?,
                        )
                    } else {
                        None
                    };

                    let argument_name = {
                        let (name_start, name_end) = match separator {
                            None => (cursor + 1, current_char),
                            Some(seperator_index) => (cursor + 1, seperator_index),
                        };

                        if name_start == name_end {
                            internal_index += 1;
                            ArgumentName::ArgumentKey(ArgumentKey::Index(internal_index - 1))
                        } else {
                            let name = &input[name_start..name_end];
                            match name.parse::<usize>() {
                                Ok(arg_index) => {
                                    ArgumentName::ArgumentKey(ArgumentKey::Index(arg_index))
                                }
                                Err(_) => {
                                    match (
                                        name.starts_with("self."),
                                        name.starts_with("root."),
                                        name.contains("::"),
                                    ) {
                                        (false, false, false) => ArgumentName::ArgumentKey(
                                            ArgumentKey::Name(name.to_string()),
                                        ),
                                        (false, false, true) => {
                                            ArgumentName::Const(NameRef::Other(name.to_string()))
                                        }
                                        (ref_self, _, _) => ArgumentName::Const(NameRef::Ast {
                                            origin: if ref_self {
                                                AstRefOrigin::SelfNode
                                            } else {
                                                AstRefOrigin::RootNode
                                            },
                                            path: name[5..]
                                                .split(".")
                                                .map(|part| match part.parse::<usize>() {
                                                    Ok(index) => Identifier::Element(index),
                                                    Err(_) => Identifier::Field(part.to_string()),
                                                })
                                                .collect(),
                                        }),
                                    }
                                }
                            }
                        }
                    };

                    pieces.push(Piece::Argument {
                        name: argument_name,
                        specifier,
                    });

                    separator = None;
                    bracket = None;
                    cursor = current_char + 1;
                }
                (Some(b'{'), b'{') => {
                    pieces.push(Piece::BracketOpen);
                    bracket = None;
                    cursor = current_char + 1;
                }
                (Some(b'}'), b'}') => {
                    pieces.push(Piece::BracketClose);
                    bracket = None;
                    cursor = current_char + 1;
                }
                _ => {
                    return Err(error());
                }
            },
            _ => {}
        }
        current_char += 1;
    }

    if cursor < current_char {
        pieces.push(Piece::Literal(input[cursor..current_char].to_string()));
    }

    if bracket.is_some() {
        Err(error())
    } else {
        Ok(pieces)
    }
}
