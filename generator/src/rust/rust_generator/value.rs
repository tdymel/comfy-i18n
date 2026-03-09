use comfy_i18n_ast::{
    Ast, CompositeValue, FloatValue, Identifier, IntegerValue, LiteralValue, NodeValue,
    StringValue, Template,
};
use quote::{ToTokens, quote};

use crate::{
    rust::{rust_generator::VariableType, shared::NameSnakeCase},
    rust_generator::{Context, Path},
    shared::ToBasicTokenStream,
};

#[derive(Debug, Clone)]
pub enum RustValue {
    String(String),
    Format {
        path: Path,
        template: Template,
        context_variant: Box<RustValue>,
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
        fields: Vec<RustValue>,
    },
    Tuple(Vec<RustValue>),
    ListRepeated {
        value: Box<RustValue>,
        amount: usize,
    },
    List(Vec<RustValue>, bool),
    Some(Box<RustValue>),
    None,
    Cast(proc_macro2::TokenStream),
}

impl ToTokens for RustValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            RustValue::String(val) => quote! { #val },
            RustValue::Format { path, template, context_variant } => {
                let mut template_str = template.to_string();
                template.arguments()
                    .iter()
                    .for_each(|(arg, _)| {
                        if arg.to_string().contains(".") {
                            template_str = template_str.replace(&arg.to_string(), &arg.to_string().replace(".", "_"));
                        }
                    });

                quote! { #path::new(#context_variant, comfy_i18n::macro_use::Template::parse(#template_str).unwrap()) }
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
                IntegerValue::Usize(val) => quote! {#val},
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
                    #path::new(
                        #(#fields),*
                    )
                }
            }
            RustValue::Tuple(values) => {
                quote! { (#(#values),*) }
            }
            RustValue::ListRepeated { value, amount } => {
                quote! { [#value; #amount] }
            }
            RustValue::List(values, is_copy) => {
                if *is_copy {
                    quote! { [#(#values),*] }
                } else {
                    quote! { vec![#(#values),*] }
                }
            }
            RustValue::Some(val) => quote! { Some(#val) },
            RustValue::None => quote! { None },
            RustValue::Cast(expr) => quote! {{ #expr }}
        });
    }
}

impl RustValue {
    pub fn new(ast: &Ast, context: &Context) -> Self {
        Self::by_variant(
            ast,
            context,
            &Identifier::Field(context.context_variant(&ast.id)),
        )
    }

    pub fn by_variant(ast_orig: &Ast, context: &Context, variant: &Identifier) -> Self {
        let ast = context.get_variant(&ast_orig.id, variant.clone());
        if ast.is_none() {
            return RustValue::None;
        }
        let ast = ast.unwrap();

        match &ast_orig.value {
            NodeValue::Composite { children, value } => {
                let mut pairs = children.iter().collect::<Vec<_>>();
                pairs.sort_by_key(|(k1, _)| *k1);
                let mut values = pairs
                    .into_iter()
                    .map(|(k, v)| (k, Self::by_variant(v, context, variant)))
                    .collect::<Vec<_>>();

                match value {
                    CompositeValue::Struct => RustValue::Struct {
                        path: context.relative_path_to_root(&ast_orig.id),
                        fields: {
                            let mut fields = values
                                .into_iter()
                                .map(|(_, v)| {
                                    if let RustValue::None = v {
                                        v
                                    } else {
                                        RustValue::Some(Box::new(v))
                                    }
                                })
                                .collect::<Vec<_>>();

                            fields.push(RustValue::ContextVariant {
                                path: context.context_key().clone(),
                                variant: variant.to_string(),
                            });

                            fields
                        },
                    },
                    CompositeValue::Tuple => RustValue::Struct {
                        path: context.relative_path_to_root(&ast_orig.id),
                        fields: vec![
                            RustValue::ContextVariant {
                                path: context.context_key().clone(),
                                variant: variant.to_string(),
                            },
                            // TODO: Handle fields that dont exist
                            RustValue::Tuple(
                                values
                                    .into_iter()
                                    .map(|(_, v)| RustValue::Some(Box::new(v)))
                                    .collect(),
                            ),
                        ],
                    },
                    CompositeValue::List { amount: list_size } => RustValue::Struct {
                        path: context.relative_path_to_root(&ast_orig.id),
                        fields: vec![
                            if values.len() == 1 {
                                let (_, value) = values.remove(0);
                                if context.is_copy(&ast_orig.id) {
                                    RustValue::ListRepeated {
                                        value: Box::new(value),
                                        amount: *list_size,
                                    }
                                } else {
                                    RustValue::List(
                                        std::iter::repeat_n(value, *list_size)
                                            .filter(|it| !matches!(it, RustValue::None))
                                            .collect(),
                                        false,
                                    )
                                }
                            } else {
                                RustValue::List(
                                    values
                                        .into_iter()
                                        .map(|(_, v)| v)
                                        .filter(|it| !matches!(it, RustValue::None))
                                        .collect(),
                                    context.is_copy(&ast_orig.id),
                                )
                            },
                        ],
                    },
                }
            }
            NodeValue::Literal(..) => match &ast.value {
                NodeValue::Literal(literal_value) => match literal_value {
                    LiteralValue::String(string_value) => match string_value {
                        StringValue::Literal(lit) => RustValue::String(lit.clone()),
                        StringValue::Template(template) => RustValue::Format {
                            path: context.relative_path_to_root(&ast.id),
                            template: template.clone(),
                            context_variant: Box::new(RustValue::ContextVariant {
                                path: context.context_key().clone(),
                                variant: variant.to_string(),
                            }),
                        },
                    },
                    LiteralValue::Char(val) => RustValue::Char(*val),
                    LiteralValue::Float(float_value) => RustValue::Float(float_value.clone()),
                    LiteralValue::Integer(integer_value) => {
                        RustValue::Integer(integer_value.clone())
                    }
                    LiteralValue::Bool(val) => RustValue::Bool(*val),
                    LiteralValue::Cast { expression, .. } => {
                        RustValue::Cast(expression.to_basic_token_stream())
                    }
                },
                NodeValue::Composite { .. } => unreachable!(),
            },
        }
    }
}
