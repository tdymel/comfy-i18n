use std::collections::HashMap;

use comfy_i18n_ast::{Ast, Identifier};
use quote::quote;

use crate::{
    components::ValueWrapper,
    rust_generator::{Context, RustType},
    shared::{NameSnakeCase, ToBasicTokenStream},
};

// TODO: Right now Format-Elements are not well supported
pub fn tuple_wrapper(
    node: &Ast,
    children: &HashMap<Identifier, Ast>,
    context: &Context,
) -> proc_macro2::TokenStream {
    let path = context.relative_path_to_root(&node.id);
    let context_key = context.context_key();
    let absolute_path = path
        .clone()
        .prepend_mod(context.root_name())
        .set_ty(node.identifier.clone().into());

    let mut pairs = children.iter().collect::<Vec<_>>();
    pairs.sort_by_key(|(k1, _)| *k1);
    let tys = pairs
        .iter()
        .map(|(_, field)| RustType::new(field, context, &path))
        .collect::<Vec<_>>();

    let elems = tys
        .iter()
        .enumerate()
        .map(|(index, ty)| {
            ValueWrapper::new(
                path.clone()
                    .prepend_mod(context.root_name())
                    .add_mod(NameSnakeCase::tuple_index(index))
                    .set_ty(format!("Elem{}", index).into()),
                context.context_key().clone(),
                context
                    .context_variants()
                    .map(|it| it.to_string())
                    .collect(),
                ty.clone(),
            )
        })
        .collect::<Vec<_>>();

    let name = absolute_path.ty().unwrap();

    let tys_w_o_option = tys
        .iter()
        .map(|ty| {
            quote! { &'static #ty }
        })
        .collect::<Vec<_>>();

    let tys = tys
        .iter()
        .map(|ty| quote! { Option<#ty> })
        .collect::<Vec<_>>();

    let wrapper_tys = tys
        .iter()
        .enumerate()
        .map(|(index, _)| format!("Elem{}", index).to_basic_token_stream())
        .collect::<Vec<_>>();

    let values_self = tys
        .iter()
        .enumerate()
        .map(|(index, _)| {
            format!("Elem{0}::new(context, value.{0})", index).to_basic_token_stream()
        })
        .collect::<Vec<_>>();

    let access_path = absolute_path.to_access_path();
    let value_getter = tys
        .iter()
        .enumerate()
        .map(|(index, _)| {
            format!("self.context.{}.{}()", access_path, index).to_basic_token_stream()
        })
        .collect::<Vec<_>>();

    let by_path_match_arms = tys
        .iter()
        .enumerate()
        .map(|(index, _)| {
            format!("\"{0}\" => self.value.{0}.by_path(path)", index).to_basic_token_stream()
        })
        .collect::<Vec<_>>();

    quote! {
        #[derive(Clone)]
        pub struct #name {
            context: #context_key,
            value: (#(#wrapper_tys),*)
        }

        impl #name {
            pub fn new(context: #context_key, value: (#(#tys),*)) -> Self {
                Self {
                    context,
                    value: (
                        #(#values_self),*
                    )
                }
            }

            pub fn value(&self) -> (#(#tys_w_o_option),*) {
                (
                    #(#value_getter),*
                )
            }

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

        impl core::ops::Deref for #name
        {
            type Target = (#(#wrapper_tys),*);

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        #(#elems)*
    }
}
