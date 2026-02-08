use comfy_i18n_ast::SpannedAst;
use comfy_i18n_parser::Parser;
use proc_macro2::Span;
use syn::{Ident, parse::Parse};

pub struct I18n {
    pub name: Ident,
    pub key: Ident,
    pub translations: SpannedAst<Span>,
}

impl Parse for I18n {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name_key = input.parse::<Ident>()?;
        assert_eq!(name_key.to_string().as_str(), "name");
        input.parse::<syn::token::Colon>()?;
        let name_value = input.parse::<Ident>()?;
        input.parse::<syn::token::Comma>()?;

        let key_key = input.parse::<Ident>()?;
        assert_eq!(key_key.to_string().as_str(), "key");
        input.parse::<syn::token::Colon>()?;
        let key_value = input.parse::<Ident>()?; // TODO
        input.parse::<syn::token::Comma>()?;

        // This is everything but robust!
        let translations = input
            .parse::<proc_macro2::TokenStream>()?
            .parse_field()
            .unwrap();

        Ok(Self {
            name: name_value,
            key: key_value,
            translations,
        })
    }
}
