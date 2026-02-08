use comfy_i18n_ast::{FloatValue, IntegerValue, Template};
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

impl Initialization {
    pub fn rename(&mut self, name: NameSnakeCase) {
        assert!(matches!(self.ty, RustType::Struct { .. }));
        assert!(matches!(self.value, RustValue::Struct { .. }));

        // TODO: This could break any time if a context contains "_"
        let context = self.name.last_part();
        self.name = name.clone().concat(NameSnakeCase::from(context));

        self.ty = RustType::Struct {
            mod_names: Vec::new(),
            name: name.to_pascal_case(),
        };

        if let RustValue::Struct { fields, .. } = &self.value {
            self.value = RustValue::Struct {
                mod_names: Vec::new(),
                name: name.to_pascal_case(),
                fields: fields.clone(),
            }
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum RustValue {
    String(String),
    Format {
        name: NamePascalCase,
        template: Template,
    },
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
            RustValue::String(val) => quote! { #val },
            RustValue::Format { name, template } => {
                let template = template.to_string();
                quote! { #name { template: #template } }
            }
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
