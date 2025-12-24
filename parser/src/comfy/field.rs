use std::collections::HashMap;

use comfy_i18n_ast::{Identifier, SpannedAst};
use proc_macro2::Span;
use syn::{
    Ident,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::comfy::value::Value;

pub struct Field(pub SpannedAst<Span>);

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let start_span = input.span();

        let name = input.parse::<Ident>()?;
        input.parse::<syn::token::Colon>()?;
        let value = input.parse::<Value>()?;

        Ok(Field(SpannedAst::new(
            Identifier::Field(name.to_string()),
            start_span.join(input.span()).unwrap_or(start_span),
            value.0,
        )))
    }
}

pub struct Fields(pub Vec<SpannedAst<Span>>);

impl Parse for Fields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Fields(
            Punctuated::<Field, syn::token::Comma>::parse_terminated(input)?
                .into_iter()
                .map(|field| field.0)
                .collect(),
        ))
    }
}

impl From<Fields> for HashMap<Identifier, SpannedAst<Span>> {
    fn from(value: Fields) -> Self {
        value
            .0
            .into_iter()
            .map(|it| (it.identifier.clone(), it))
            .collect::<HashMap<Identifier, SpannedAst<Span>>>()
    }
}
