use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use crate::comfy_i18n::ComfyI18n;
use crate::i18n::I18n;

mod comfy_i18n;
mod i18n;

#[proc_macro]
pub fn i18n(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as I18n).to_token_stream();

    result.into()
}

#[proc_macro_derive(ComfyI18n, attributes(fallback))]
pub fn comfy_i18n(input: TokenStream) -> TokenStream {
    let result = parse_macro_input!(input as ComfyI18n).to_token_stream();

    result.into()
}
