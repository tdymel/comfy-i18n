use comfy_i18n_ast::Ast;
use quote::quote;

use crate::{
    components::{fallback_fn, hackfn},
    rust_generator::{Context, Path, RustType},
    shared::ToBasicTokenStream,
};

pub fn value_wrapper(
    node: &Ast,
    context: &Context,
    absolute_path: Path,
    context_path: Path,
    available_variants: Vec<String>,
    ty: RustType,
) -> proc_macro2::TokenStream {
    let name = absolute_path.ty().unwrap();
    let mut access_path = absolute_path.to_access_path();
    if access_path.ends_with("()") {
        access_path = access_path[..access_path.len() - 2].to_string();
    }
    let access_path = access_path.to_basic_token_stream();
    let hackfn = hackfn(
        name,
        &"_self".into(),
        &Vec::new(),
        &Vec::new(),
        quote! { &'static #ty},
    );

    let fallback_fn = fallback_fn(
        true,
        &"_self".into(),
        None,
        &ty,
        &context_path,
        access_path,
        "value.as_ref().unwrap()".to_basic_token_stream(),
        &available_variants,
    );

    let by_path_fn = match &ty {
        RustType::Struct(_) | RustType::Tuple(_) | RustType::List { .. } => {
            quote! { 
                pub fn by_path(
                    &'static self,
                    path: std::collections::VecDeque<String>,
                ) -> &'static (dyn std::any::Any + Sync) {
                    self._self().by_path(path) 
                }
            }
        }
        _ => quote! { 
            pub fn by_path(
                &'static self,
                _path: std::collections::VecDeque<String>,
            ) -> &'static (dyn std::any::Any + Sync) {
                self._self() 
            }
        },
    };

    let is_copy = if context.is_copy(&node.id) {
        quote! { , Copy }
    } else {
        quote! {}
    };

    quote! {
        #[derive(Clone #is_copy)]
        pub struct #name {
            context: #context_path,
            value: Option<#ty>
        }

        impl #name {
            const fn new(context: #context_path, value: Option<#ty>) -> Self {
                Self { context, value }
            }

            #fallback_fn

            #by_path_fn
        }

        #hackfn
    }
}
