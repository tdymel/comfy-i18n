use std::collections::HashMap;

use comfy_i18n_ast::{Ast, Identifier};
use quote::quote;

use crate::{
    rust_generator::{Context, RustType},
    shared::NamePascalCase,
};

pub fn array_wrapper(
    node: &Ast,
    children: &HashMap<Identifier, Ast>,
    context: &Context,
) -> proc_macro2::TokenStream {
    let path = context.relative_path_to_root(&node.id);
    let name: NamePascalCase = node.identifier.clone().into();
    let ty = RustType::new(
        children.get(&Identifier::ArrayIndex(0)).unwrap(),
        context,
        &path,
    );

    let by_path_return_value = match &ty {
        RustType::Struct(_) | RustType::Tuple(_) | RustType::List { .. } => {
            quote! { self[index].by_path(path) }
        }
        _ => quote! { &self[index] },
    };

    quote! {
        #[derive(Clone)]
        pub struct #name {
            value: Vec<#ty>
        }

        impl #name {
            pub fn new(value: Vec<#ty>) -> Self {
                Self { value }
            }

            pub fn value(&'static self) -> &'static Vec<#ty> {
                &self.value
            }

            pub fn by_path(
                &'static self,
                mut path: std::collections::VecDeque<String>,
            ) -> &'static (dyn std::any::Any + Sync) {
                if path.is_empty() {
                    return self.value();
                }
                let index = path.pop_front().unwrap().parse::<usize>().unwrap();
                #by_path_return_value
            }
        }

        impl core::ops::Index<usize> for #name {
            type Output = #ty;

            fn index(&self, index: usize) -> &Self::Output {
                &self.value[index]
            }
        }
    }
}
