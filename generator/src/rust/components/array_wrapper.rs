use std::collections::HashMap;

use comfy_i18n_ast::{Ast, CompositeValue, Identifier, NodeValue};
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

    let mut copy = quote! {};
    // TODO: If the lists in all value contexts have a different length, its also not copy
    let list_ty;
    if context.is_copy(&node.id) {
        let list_size = if let NodeValue::Composite {
            value: CompositeValue::List { amount },
            ..
        } = &node.value
        {
            *amount
        } else {
            children.len()
        };

        copy = quote! { , Copy };
        list_ty = quote! { [#ty; #list_size] };
    } else {
        list_ty = quote! { Vec<#ty> };
    }

    quote! {
        #[derive(Clone #copy)]
        pub struct #name {
            value: #list_ty
        }

        impl #name {
            pub fn new(value: #list_ty) -> Self {
                Self { value }
            }

            pub fn value(&'static self) -> &'static #list_ty {
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
