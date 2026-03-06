use quote::{ToTokens, quote};

use crate::{
    components::{fallback_fn, hackfn},
    rust_generator::{Path, RustType},
    shared::ToBasicTokenStream,
};

pub struct ValueWrapper {
    absolute_path: Path,
    context_path: Path,
    available_variants: Vec<String>,
    ty: RustType,
}

impl ValueWrapper {
    pub const fn new(
        absolute_path: Path,
        context_path: Path,
        available_variants: Vec<String>,
        ty: RustType,
    ) -> Self {
        Self {
            absolute_path,
            context_path,
            available_variants,
            ty,
        }
    }
}

impl ToTokens for ValueWrapper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.absolute_path.ty().unwrap();
        let context_path = &self.context_path;
        let ty = &self.ty;

        let mut access_path = self.absolute_path.to_access_path();
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
            &"_self".into(),
            ty,
            context_path,
            access_path,
            "value.as_ref().unwrap()".to_basic_token_stream(),
            &self.available_variants,
        );

        let by_path_return_value = match &self.ty {
            // TODO: Format?!
            RustType::Struct(_) | RustType::Tuple(_) | RustType::List { .. } => {
                quote! { self._self().by_path(path) }
            }
            _ => quote! { self._self() },
        };

        tokens.extend(quote! {
            #[derive(Clone)]
            pub struct #name {
                context: #context_path,
                value: Option<#ty>
            }

            impl #name {
                const fn new(context: #context_path, value: Option<#ty>) -> Self {
                    Self { context, value }
                }

                #fallback_fn

                pub fn by_path(
                    &'static self,
                    path: std::collections::VecDeque<String>,
                ) -> &'static (dyn std::any::Any + Sync) {
                    #by_path_return_value
                }
            }

            #hackfn
        });
    }
}
