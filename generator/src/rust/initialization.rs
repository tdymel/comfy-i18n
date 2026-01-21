use comfy_i18n_ast::{FloatValue, IntegerValue, StringValue};
use quote::{ToTokens, quote};

use crate::rust::{
    name::{NamePascalCase, NameSnakeCase},
    strct::RustType,
    utils::ToBasicTokenSreamVec,
};

#[derive(Debug)]
pub struct Initialization {
    pub var_ty: VariableType,
    pub ty: RustType,
    pub name: NameSnakeCase,
    pub value: RustValue,
}

impl ToTokens for Initialization {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let var_ty = self.var_ty.to_token_stream();
        let ty = self.ty.to_token_stream();
        let name = self.name.to_uppercase().to_token_stream();
        let value = self.value.to_token_stream();

        if self.var_ty == VariableType::Static {
            tokens.extend(quote! {
                pub #var_ty #name: std::sync::LazyLock<#ty> = std::sync::LazyLock::new(|| #value);
            });
        } else {
            tokens.extend(quote! {
                pub #var_ty #name: #ty = #value;
            });
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VariableType {
    Const,
    Static,
}

impl ToTokens for VariableType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            VariableType::Const => quote! { const },
            VariableType::Static => quote! { static },
        });
    }
}

#[derive(Debug)]
pub struct FieldValue {
    pub name: NameSnakeCase,
    pub value: RustValue,
}

impl ToTokens for FieldValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.name.to_lowercase().to_token_stream();
        let value = self.value.to_token_stream();

        tokens.extend(quote! {
            #name: #value
        });
    }
}

#[derive(Debug)]
pub enum RustValue {
    String(StringValue),
    Char(char),
    Float(FloatValue),
    Integer(IntegerValue),
    Bool(bool),
    Reference {
        var_ty: VariableType,
        mod_name: NameSnakeCase,
        var_name: NameSnakeCase,
    },
    Struct {
        mod_names: Vec<NameSnakeCase>,
        name: NamePascalCase,
        fields: Vec<FieldValue>,
    },
    Tuple(Vec<RustValue>),
    ListRepeated {
        value: Box<RustValue>,
        amount: usize,
    },
    List(Vec<RustValue>),
}

impl ToTokens for RustValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            RustValue::String(string_value) => match string_value {
                StringValue::Literal(val) => quote! { #val },
                StringValue::Template(_template) => todo!(),
            },
            RustValue::Char(val) => quote! {#val},
            RustValue::Float(float_value) => match float_value {
                FloatValue::F64(val) => quote! { #val },
                FloatValue::F32(val) => quote! { #val },
            },
            RustValue::Integer(integer_value) => match integer_value {
                IntegerValue::I128(val) => quote! {#val},
                IntegerValue::U128(val) => quote! {#val},
                IntegerValue::I64(val) => quote! {#val},
                IntegerValue::U64(val) => quote! {#val},
                IntegerValue::I32(val) => quote! {#val},
                IntegerValue::U32(val) => quote! {#val},
                IntegerValue::I16(val) => quote! {#val},
                IntegerValue::U16(val) => quote! {#val},
                IntegerValue::I8(val) => quote! {#val},
                IntegerValue::U8(val) => quote! {#val},
            },
            RustValue::Bool(val) => quote! {#val},
            RustValue::Reference {
                var_ty,
                mod_name,
                var_name,
            } => {
                let mod_name = mod_name.to_lowercase().to_token_stream();
                let var_name = var_name.to_uppercase().to_token_stream();
                if var_ty == &VariableType::Static {
                    quote! { #mod_name :: #var_name.clone() }
                } else {
                    quote! { #mod_name :: #var_name }
                }
            }
            RustValue::Struct {
                mod_names,
                name,
                fields,
            } => {
                let name = name.to_token_stream();
                let fields = fields.to_token_stream();

                if mod_names.is_empty() {
                    quote! {
                        #name {
                            #(#fields),*
                        }
                    }
                } else {
                    let mod_names = mod_names.to_token_stream();
                    quote! {
                        #(#mod_names)::* :: #name {
                            #(#fields),*
                        }
                    }
                }
            }
            RustValue::Tuple(values) => {
                let values = values.to_token_stream();
                quote! { (#(#values),*) }
            }
            RustValue::ListRepeated { value, amount } => {
                let value = value.to_token_stream();
                quote! { [#value; #amount] }
            }
            RustValue::List(values) => {
                let values = values.to_token_stream();
                quote! { vec![#(#values),*] }
            }
        });
    }
}
