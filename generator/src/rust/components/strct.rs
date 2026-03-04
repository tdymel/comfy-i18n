use std::collections::HashMap;

use comfy_i18n_ast::{Ast, Identifier, LiteralValue, NodeValue, StringValue};
use quote::{ToTokens, quote};

use crate::{
    components::fallback_fn,
    rust::{
        rust_generator::RustType,
        shared::{NamePascalCase, NameSnakeCase},
    },
    rust_generator::Context,
    shared::ToBasicTokenStream,
};

pub fn strct(
    node: &Ast,
    children: &HashMap<Identifier, Ast>,
    context: &Context,
    with_comfy_i18n_context: bool,
) -> proc_macro2::TokenStream {
    let path = context.relative_path_to_root(&node.id);
    let strct_name = if path.has_no_mods() {
        context.root_name().to_pascal_case()
    } else {
        node.identifier.clone().into()
    };

    let mut fields: Vec<Field> = children
        .values()
        .map(|field| {
            Field::optional(
                field.identifier.clone().into(),
                RustType::new(field, context, &path),
            )
        })
        .collect::<Vec<_>>();
    fields.sort_by_key(|f1| f1.name.to_string());
    if with_comfy_i18n_context {
        fields.push(Field::new(
            "comfy_i18n_context".into(),
            RustType::Other(context.context_key().clone()),
        ));
    }

    let absolute_path = path
        .clone()
        .prepend_mod(context.root_name())
        .set_ty(node.identifier.clone().into());

    let fns = children
        .values()
        .filter(|ast| {
            !matches!(
                ast.value,
                NodeValue::Literal(LiteralValue::String(StringValue::Template(..)))
            )
        })
        .map(|ast| {
            let name = NameSnakeCase::from(ast.identifier.clone());
            fallback_fn(
                &name,
                &RustType::new(ast, context, &path),
                &context.context_key(),
                absolute_path.to_access_path().to_basic_token_stream(),
                quote! { #name.as_ref().unwrap() },
                &context
                    .available_context_variants(&ast.id)
                    .collect::<Vec<String>>(),
            )
        })
        .collect::<Vec<_>>();

    let field_names = fields.iter().map(|it| it.name.clone()).collect::<Vec<_>>();

    let new_tys = fields
        .iter()
        .map(|it| Field {
            name: it.name.clone(),
            optional: it.optional,
            ty: it.ty.clone(),
            public: false,
        })
        .collect::<Vec<_>>();

    let by_path_match_arms = fields
        .iter()
        .filter(|it| !it.name.to_string().contains("comfy_i18n"))
        .map(|field| {
            match &field.ty {
                RustType::Format(_) => format!("\"{0}\" => self.{0}_value()", field.name),
                RustType::Struct(_) | RustType::Tuple(_) | RustType::List { .. } => {
                    format!("\"{0}\" => self.{0}().by_path(path)", field.name)
                }
                _ => format!("\"{0}\" => self.{0}()", field.name),
            }
            .to_basic_token_stream()
        })
        .collect::<Vec<_>>();

    // TODO: Not every type will support copy! Validation!
    quote! {
        #[derive(Clone)]
        pub struct #strct_name {
            #(#fields),*
        }

        impl #strct_name {
            pub const fn new(
                #(#new_tys),*
            ) -> Self {
                Self {
                    #(#field_names),*
                }
            }

            #(#fns)*

            pub fn by_path(
                &'static self,
                mut path: std::collections::VecDeque<String>,
            ) -> &'static (dyn std::any::Any + Sync) {
                if path.is_empty() {
                    return self;
                }
                let key = path.pop_front().unwrap();

                match key.as_str() {
                    #(#by_path_match_arms,)*
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Struct {
    pub name: NamePascalCase,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: NamePascalCase, mut fields: Vec<Field>) -> Self {
        fields.sort_by_key(|f1| f1.name.to_string());
        Self { name, fields }
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let fields = &self.fields;
        let field_names = self
            .fields
            .iter()
            .map(|it| it.name.clone())
            .collect::<Vec<_>>();

        let new_tys = self
            .fields
            .iter()
            .map(|it| Field {
                name: it.name.clone(),
                optional: it.optional,
                ty: it.ty.clone(),
                public: false,
            })
            .collect::<Vec<_>>();

        // TODO: Not every type will support copy! Validation!
        tokens.extend(quote! {
            #[derive(Clone)]
            pub struct #name {
                #(#fields),*
            }

            impl #name {
                pub const fn new(
                    #(#new_tys),*
                ) -> Self {
                    Self {
                        #(#field_names),*
                    }
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: NameSnakeCase,
    pub ty: RustType,
    pub optional: bool,
    pub public: bool,
}

impl Field {
    pub fn optional(name: NameSnakeCase, ty: RustType) -> Self {
        Self {
            name,
            ty,
            optional: true,
            public: false,
        }
    }

    pub fn new(name: NameSnakeCase, ty: RustType) -> Self {
        Self {
            name,
            ty,
            optional: false,
            public: false,
        }
    }

    pub fn public(name: NameSnakeCase, ty: RustType) -> Self {
        Self {
            name,
            ty,
            optional: false,
            public: true,
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name.to_lowercase();
        let type_name = &self.ty;

        if self.optional {
            tokens.extend(quote! {
                #name: Option<#type_name>
            });
        } else {
            if self.public {
                tokens.extend(quote! {
                    pub #name: #type_name
                });
            } else {
                tokens.extend(quote! {
                    #name: #type_name
                });
            }
        }
    }
}
