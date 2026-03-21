use comfy_i18n_ast::Ast;
use quote::quote;

use crate::{
    components::{fallback_fn, hackfn, to_fmt_args},
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
    let fmt_args = if let RustType::Format(_, template) = &ty {
        Some(to_fmt_args(template))
    } else {
        None
    };
    let hackfn_ty = if let RustType::Format(..) = &ty {
        quote! { String }
    } else {
        quote! { &'static #ty}
    };
    let hackfn = hackfn(
        name,
        &"_self".into(),
        // Not clean at all, but it works
        &fmt_args.clone().map(|it| it.1).unwrap_or_else(Vec::new),
        &fmt_args.map(|it| it.0).unwrap_or_else(Vec::new),
        quote! { #hackfn_ty},
    );

    // Also not really clean
    let fallback_fn_value = fallback_fn(
        true,
        &"_self_value".into(),
        None,
        &ty,
        &context_path,
        access_path.clone(),
        "value.as_ref().unwrap()".to_basic_token_stream(),
        &available_variants,
        false,
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
        true,
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
                self._self_value()
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

            #fallback_fn_value

            #fallback_fn

            #by_path_fn
        }

        #hackfn
    }
}
