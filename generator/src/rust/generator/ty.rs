use comfy_i18n_ast::{
    Ast, CompositeValue, FloatValue, Identifier, IntegerValue, LiteralValue, NodeValue, StringValue,
};
use quote::{ToTokens, quote};

use crate::{
    generator::{Context, Path},
    rust::shared::ToBasicTokenStream,
};

#[derive(Debug, Clone)]
pub enum RustType {
    String,
    Char,
    Bool,
    Float {
        bits: u8,
    },
    Integer {
        unsigned: bool,
        bits: u8,
    },
    List {
        ty: Box<RustType>,
        amount: Option<usize>,
    },
    Tuple(Path),
    Format(Path),
    Struct(Path),
    Other(Path),
}

impl ToTokens for RustType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            RustType::String => quote! { &'static str },
            RustType::Char => quote! { char },
            RustType::Bool => quote! { bool },
            RustType::Float { bits } => match bits {
                32 | 64 => {
                    let type_name = format!("f{}", bits).to_basic_token_stream();
                    quote! { #type_name }
                }
                _ => panic!(),
            },
            RustType::Integer { unsigned, bits } => match bits {
                8 | 16 | 32 | 64 | 128 => {
                    let type_name = match unsigned {
                        true => format!("u{}", bits),
                        false => format!("i{}", bits),
                    }
                    .to_basic_token_stream();
                    quote! { #type_name }
                }
                _ => panic!(),
            },
            RustType::List {
                ty: rust_type,
                amount,
            } => {
                if let Some(amount) = amount {
                    quote! { [#rust_type; #amount]}
                } else {
                    quote! {Vec<#rust_type>}
                }
            }
            RustType::Format(path)
            | RustType::Tuple(path)
            | RustType::Struct(path)
            | RustType::Other(path) => path.to_token_stream(),
        });
    }
}

impl RustType {
    pub fn new(node: &Ast, context: &Context, relative_root: &Path) -> Self {
        match &node.value {
            NodeValue::Composite { children, value } => match value {
                CompositeValue::Struct => Self::Struct(
                    context
                        .relative_path_to_root(&node.id)
                        .relative_to(relative_root),
                ),
                CompositeValue::Tuple => Self::Tuple(
                    context
                        .relative_path_to_root(&node.id)
                        .relative_to(relative_root),
                ),
                CompositeValue::List { amount: list_size } => {
                    let ty = Self::new(
                        children.get(&Identifier::ArrayIndex(0)).unwrap(),
                        context,
                        relative_root,
                    );
                    RustType::List {
                        ty: Box::new(ty),
                        amount: Some(*list_size),
                    }
                }
            },
            NodeValue::Literal(literal_value) => match literal_value {
                LiteralValue::String(string_value) => match string_value {
                    StringValue::Literal(_) => RustType::String,
                    StringValue::Template(..) => {
                        RustType::Format(context.relative_path_to_root(&node.id))
                    }
                },
                LiteralValue::Char(_) => RustType::Char,
                LiteralValue::Float(float_value) => match float_value {
                    FloatValue::F64(_) => RustType::Float { bits: 64 },
                    FloatValue::F32(_) => RustType::Float { bits: 32 },
                },
                LiteralValue::Integer(integer_value) => match integer_value {
                    IntegerValue::I128(_) => RustType::Integer {
                        unsigned: false,
                        bits: 128,
                    },
                    IntegerValue::U128(_) => RustType::Integer {
                        unsigned: true,
                        bits: 128,
                    },
                    IntegerValue::I64(_) => RustType::Integer {
                        unsigned: false,
                        bits: 64,
                    },
                    IntegerValue::U64(_) => RustType::Integer {
                        unsigned: true,
                        bits: 64,
                    },
                    IntegerValue::I32(_) => RustType::Integer {
                        unsigned: false,
                        bits: 32,
                    },
                    IntegerValue::U32(_) => RustType::Integer {
                        unsigned: true,
                        bits: 32,
                    },
                    IntegerValue::I16(_) => RustType::Integer {
                        unsigned: false,
                        bits: 16,
                    },
                    IntegerValue::U16(_) => RustType::Integer {
                        unsigned: true,
                        bits: 16,
                    },
                    IntegerValue::I8(_) => RustType::Integer {
                        unsigned: false,
                        bits: 8,
                    },
                    IntegerValue::U8(_) => RustType::Integer {
                        unsigned: true,
                        bits: 8,
                    },
                },
                LiteralValue::Bool(_) => RustType::Bool,
                LiteralValue::Cast { .. } => todo!(),
            },
        }
    }
}
