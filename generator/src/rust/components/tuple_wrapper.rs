use quote::{ToTokens, quote};

use crate::{
    generator::{Path, RustType},
    shared::ToBasicTokenStream,
};

pub struct TupleWrapper {
    absolute_path: Path,
    context_path: Path,
    tys: Vec<RustType>,
}

impl TupleWrapper {
    pub fn new(absolute_path: Path, context_path: Path, tys: Vec<RustType>) -> Self {
        Self {
            absolute_path,
            context_path,
            tys,
        }
    }
}

impl ToTokens for TupleWrapper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.absolute_path.ty().unwrap();
        let context_path = &self.context_path;
        let tys_w_o_option = &self.tys;

        let tys = self
            .tys
            .iter()
            .map(|ty| quote! { Option<#ty> })
            .collect::<Vec<_>>();

        let wrapper_tys = self
            .tys
            .iter()
            .enumerate()
            .map(|(index, _)| format!("Elem{}", index).to_basic_token_stream())
            .collect::<Vec<_>>();

        let values_self = self
            .tys
            .iter()
            .enumerate()
            .map(|(index, _)| {
                format!("Elem{0}::new(comfy_i18n_context, value.{0})", index)
                    .to_basic_token_stream()
            })
            .collect::<Vec<_>>();

        let access_path = self.absolute_path.to_access_path();
        let value_getter = self
            .tys
            .iter()
            .enumerate()
            .map(|(index, _)| format!("self.comfy_i18n_context.{}.{}()", access_path, index).to_basic_token_stream())
            .collect::<Vec<_>>();

        tokens.extend(quote! {
            #[derive(Clone, Copy)]
            pub struct #name {
                comfy_i18n_context: #context_path,
                value: (#(#wrapper_tys),*)
            }

            impl #name {
                pub const fn new(comfy_i18n_context: #context_path, value: (#(#tys),*)) -> Self {
                    Self {
                        comfy_i18n_context,
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
            }

            impl core::ops::Deref for #name
            {
                type Target = (#(#wrapper_tys),*);

                fn deref(&self) -> &Self::Target {
                    &self.value
                }
            }
        });
    }
}
