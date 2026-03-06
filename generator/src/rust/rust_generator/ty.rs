use comfy_i18n_ast::{
    Ast, CompositeValue, FloatValue, IntegerValue, LiteralValue, NodeValue, StringValue,
};
use quote::{ToTokens, quote};

use crate::{
    rust::shared::ToBasicTokenStream,
    rust_generator::{Context, Path},
};

#[derive(Debug, Clone)]
pub enum RustType {
    String,
    Char,
    Bool,
    Float { bits: u8 },
    Usize,
    Integer { unsigned: bool, bits: u8 },
    List(Path),
    Tuple(Path),
    Format(Path),
    Struct(Path),
    Other(Path),
    Cast(proc_macro2::TokenStream),
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
            RustType::Usize => quote! {usize},
            RustType::Integer { unsigned, bits } => match bits {
                8 | 16 | 32 | 64 | 128 => {
                    let type_name = match unsigned {
                        true => format!("u{}", bits),
                        false => format!("i{}", bits),
                    }
                    .to_basic_token_stream();
                    quote! { #type_name }
                }
                _ => unreachable!(),
            },
            RustType::Format(path)
            | RustType::Tuple(path)
            | RustType::List(path)
            | RustType::Struct(path)
            | RustType::Other(path) => path.to_token_stream(),
            RustType::Cast(ty) => ty.clone(),
        });
    }
}

impl RustType {
    pub fn new(node: &Ast, context: &Context, relative_root: &Path) -> Self {
        match &node.value {
            NodeValue::Composite { children: _, value } => match value {
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
                CompositeValue::List { .. } => Self::List(
                    context
                        .relative_path_to_root(&node.id)
                        .relative_to(relative_root),
                ),
                // CompositeValue::List { amount: list_size } => {
                //     let ty = Self::new(
                //         children.get(&Identifier::ArrayIndex(0)).unwrap(),
                //         context,
                //         relative_root,
                //     );
                //     RustType::List {
                //         ty: Box::new(ty),
                //         amount: Some(*list_size),
                //     }
                // }
            },
            NodeValue::Literal(literal_value) => match literal_value {
                LiteralValue::String(string_value) => match string_value {
                    StringValue::Literal(_) => RustType::String,
                    StringValue::Template(..) => RustType::Format(
                        context
                            .relative_path_to_root(&node.id)
                            .relative_to(relative_root),
                    ),
                },
                LiteralValue::Char(_) => RustType::Char,
                LiteralValue::Float(float_value) => match float_value {
                    FloatValue::F64(_) => RustType::Float { bits: 64 },
                    FloatValue::F32(_) => RustType::Float { bits: 32 },
                },
                LiteralValue::Integer(integer_value) => match integer_value {
                    IntegerValue::Usize(_) => RustType::Usize,
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
                LiteralValue::Cast { ty, .. } => RustType::Cast(ty.to_basic_token_stream()),
            },
        }
    }
}
