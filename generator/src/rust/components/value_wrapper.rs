use quote::{ToTokens, quote};

use crate::{
    generator::{Path, RustType},
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

        let contexts = self
            .available_variants
            .iter()
            .enumerate()
            .map(|(index, context)| {
                format!(
                    "contexts[{}] = Some({}::{});",
                    index, self.context_path, context
                )
                .to_basic_token_stream()
            })
            .collect::<Vec<_>>();

        let mut access_path = self.absolute_path.to_access_path();
        if access_path.ends_with("()") {
            access_path = access_path[..access_path.len() - 2].to_string();
        }
        let access_path = access_path.to_basic_token_stream();

        tokens.extend(quote! {
            #[derive(Clone, Copy)]
            pub struct #name {
                comfy_i18n_context: #context_path,
                value: Option<#ty>
            }

            impl #name {
                const fn new(comfy_i18n_context: #context_path, value: Option<#ty>) -> Self {
                    Self { comfy_i18n_context, value }
                }

                fn _self(&self) -> #ty {
                    let mut contexts = [None; #context_path::amount()];
                    #(#contexts)*
                    self.comfy_i18n_context.fallback(contexts).#access_path.value.unwrap()
                }
            }

            impl core::ops::Deref for #name
            {
                type Target = dyn Fn() -> #ty;

                fn deref(&self) -> &Self::Target {
                    let uninit_callable: Self = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                    let uninit_closure = move || Self::_self(&uninit_callable);
                    let size_of_closure = core::mem::size_of_val(&uninit_closure);
                    fn second<'a, T>(_a: &T, b: &'a T) -> &'a T {
                        b
                    }
                    let reference_to_closure = second(&uninit_closure, unsafe { core::mem::transmute(self) });
                    core::mem::forget(uninit_closure);
                    assert_eq!(size_of_closure, core::mem::size_of::<Self>());
                    let reference_to_trait_object = reference_to_closure as &Self::Target;
                    reference_to_trait_object
                }
            }
        });
    }
}
