use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use crate::comfy_i18n::ComfyI18n;
use crate::i18n::I18n;
use crate::i18n_init::I18nInit;
use crate::t::T;

mod comfy_i18n;
mod i18n;
mod i18n_init;
mod t;

#[proc_macro]
pub fn i18n(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as I18n).to_token_stream().into()
}

#[proc_macro]
pub fn i18n_init(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as I18nInit)
        .to_token_stream()
        .into()
}

#[proc_macro]
pub fn t(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as T).to_token_stream().into()
}
