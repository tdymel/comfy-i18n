use comfy_i18n_ast::Ast;
use comfy_i18n_generator::RustGenerator;
use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::i18n::I18n;

mod i18n;

#[proc_macro]
pub fn i18n(input: TokenStream) -> TokenStream {
    let i18n = parse_macro_input!(input as I18n);
    Ast::from(i18n.translations)
        .to_rust(i18n.name.to_string())
        .into()
}
