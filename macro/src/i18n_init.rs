use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Token};

use crate::ComfyI18n;
use crate::comfy_i18n::Variant;

pub struct I18nInit {
    variants: Vec<Variant>,
}

impl Parse for I18nInit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variants = Punctuated::<Variant, Token![,]>::parse_terminated_with(input, |input| {
            let attributes = input.call(Attribute::parse_outer)?;
            let ident: Ident = input.parse()?;
            let fallback = attributes
                .iter()
                .any(|attr| attr.meta.path().is_ident("fallback"));
            Ok(Variant {
                name: ident,
                fallback,
            })
        })?;

        Ok(I18nInit {
            variants: variants.into_iter().collect(),
        })
    }
}

impl ToTokens for I18nInit {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let variants = self.variants.iter().map(|it| &it.name);
        tokens.extend(quote! {
            #[derive(Debug)]
            pub enum I18n {
                #(#variants),*
            }
        });
        
        ComfyI18n {
            name: Ident::new("I18n", Span::call_site()),
            variants: self.variants.clone(),
        }
        .to_tokens(tokens);
    }
}
