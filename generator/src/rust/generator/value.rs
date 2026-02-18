use comfy_i18n_ast::{
    Ast, CompositeValue, FloatValue, IntegerValue, LiteralValue, NodeValue, StringValue, Template,
};
use quote::{ToTokens, quote};

use crate::{
    generator::{Context, Path},
    rust::{generator::VariableType, shared::NameSnakeCase},
    shared::ToBasicTokenStream,
};

#[derive(Debug, Clone)]
pub enum RustValue {
    String(String),
    Format {
        path: Path,
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
    ContextVariant {
        path: Path,
        variant: String,
    },
    Struct {
        path: Path,
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
            RustValue::Format { path, template } => {
                let template = template.to_string();
                quote! { #path { template: #template } }
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
                let mod_name = mod_name.to_lowercase();
                let var_name = var_name.to_uppercase();
                if var_ty == &VariableType::Static {
                    quote! { #mod_name :: #var_name.clone() }
                } else {
                    quote! { #mod_name :: #var_name }
                }
            }
            RustValue::ContextVariant { path, variant } => {
                let variant = variant.to_basic_token_stream();
                quote! {
                    #path :: #variant
                }
            }
            RustValue::Struct { path, fields } => {
                quote! {
                    #path {
                        #(#fields),*
                    }
                }
            }
            RustValue::Tuple(values) => {
                quote! { (#(#values),*) }
            }
            RustValue::ListRepeated { value, amount } => {
                quote! { [#value; #amount] }
            }
            RustValue::List(values) => {
                quote! { vec![#(#values),*] }
            }
        });
    }
}

impl RustValue {
    pub fn new(ast: &Ast, context: &Context) -> Self {
        match &ast.value {
            NodeValue::Composite { children, value } => {
                let mut pairs = children.iter().collect::<Vec<_>>();
                pairs.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
                let mut values = pairs
                    .into_iter()
                    .map(|(k, v)| (k, Self::new(v, context)))
                    .collect::<Vec<_>>();

                match value {
                    CompositeValue::Struct => RustValue::Struct {
                        path: context.relative_path(&ast.id),
                        fields: {
                            let mut fields = values
                                .into_iter()
                                .map(|(k, v)| FieldValue::optional(k.clone().into(), v))
                                .collect::<Vec<_>>();

                            fields.push(FieldValue::new(
                                "comfyi18n_context".into(),
                                RustValue::ContextVariant {
                                    path: context.context_key().clone(),
                                    variant: context.context_variant(&ast.id),
                                },
                            ));

                            fields
                        },
                    },
                    CompositeValue::Tuple => {
                        RustValue::Tuple(values.into_iter().map(|(_, v)| v).collect())
                    }
                    CompositeValue::List { amount: list_size } => {
                        // let path = self.relative_path(&ast.id);
                        // let amount_the_same = self.contexts.iter().all(|context| {
                        //     matches!(
                        //         context.by_path(&path).map(|it| &it.value),
                        //         Some(&NodeValue::Composite {
                        //             value: CompositeValue::List { amount },
                        //             ..
                        //         }) if amount == *list_size
                        //     )
                        // });

                        // if amount_the_same {
                        let (_, value) = values.remove(0);
                        RustValue::ListRepeated {
                            value: Box::new(value),
                            amount: *list_size,
                        }
                        // } else {
                        // RustValue::List(values.into_iter().map(|(_, v)| v).collect())
                        // }
                    }
                }
            }
            NodeValue::Literal(literal_value) => match literal_value {
                LiteralValue::String(string_value) => match string_value {
                    StringValue::Literal(lit) => RustValue::String(lit.clone()),
                    StringValue::Template(template) => RustValue::Format {
                        path: context.relative_path(&ast.id),
                        template: template.clone(),
                    },
                },
                LiteralValue::Char(val) => RustValue::Char(*val),
                LiteralValue::Float(float_value) => RustValue::Float(float_value.clone()),
                LiteralValue::Integer(integer_value) => RustValue::Integer(integer_value.clone()),
                LiteralValue::Bool(val) => RustValue::Bool(*val),
                LiteralValue::Cast { expression: _ } => todo!(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldValue {
    pub name: NameSnakeCase,
    pub value: RustValue,
    pub optional: bool,
}

impl FieldValue {
    pub fn optional(name: NameSnakeCase, value: RustValue) -> Self {
        Self {
            name,
            value,
            optional: true,
        }
    }

    pub fn new(name: NameSnakeCase, value: RustValue) -> Self {
        Self {
            name,
            value,
            optional: false,
        }
    }
}

impl ToTokens for FieldValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.name.to_lowercase().to_token_stream();
        let value = self.value.to_token_stream();

        if self.optional {
            tokens.extend(quote! {
                #name: Some(#value)
            });
        } else {
            tokens.extend(quote! {
                #name: #value
            });
        }
    }
}
