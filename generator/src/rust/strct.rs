use quote::{ToTokens, quote};

use crate::rust::{
    name::{NamePascalCase, NameSnakeCase},
    utils::{ToBasicTokenSreamVec, ToBasicTokenStream},
};

#[derive(Debug)]
pub struct Struct {
    pub name: NamePascalCase,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: NamePascalCase) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    pub fn with_fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = fields;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.is_empty() {
            return;
        }

        let name = self.name.to_token_stream();
        let fields = self.fields.to_token_stream();

        // TODO: Not every type will support copy! Validation!
        tokens.extend(quote! {
            #[derive(Clone, Copy)]
            pub struct #name {
                #(#fields),*
            }
        });
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: NameSnakeCase,
    pub ty: RustType,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.name.to_lowercase().to_token_stream();
        let type_name = self.ty.to_token_stream();

        tokens.extend(quote! {
            pub #name: #type_name
        });
    }
}

#[derive(Debug)]
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
    Tuple(Vec<RustType>),
    List {
        ty: Box<RustType>,
        amount: Option<usize>,
    },
    Struct {
        mod_names: Vec<NameSnakeCase>,
        name: NamePascalCase,
    },
    Other {
        mod_names: Vec<NameSnakeCase>,
        name: NamePascalCase,
    },
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
            RustType::Tuple(rust_types) => {
                let type_names = rust_types.to_token_stream();
                quote! {
                    (#(#type_names),*)
                }
            }
            RustType::List {
                ty: rust_type,
                amount,
            } => {
                let type_name = rust_type.to_token_stream();
                if let Some(amount) = amount {
                    quote! { [#type_name; #amount]}
                } else {
                    quote! {Vec<#type_name>}
                }
            }
            RustType::Struct { name, mod_names } | RustType::Other { name, mod_names } => {
                let type_name = name.to_token_stream();

                if mod_names.is_empty() {
                    quote! { #type_name }
                } else {
                    let mod_names = mod_names.to_token_stream();
                    quote! { #(#mod_names)::* :: #type_name }
                }
            }
        });
    }
}
