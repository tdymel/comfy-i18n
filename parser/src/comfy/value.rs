use comfy_i18n_ast::{NodeValue, SpannedAst};
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};

use crate::comfy::{
    composite::{List, Struct, Tuple},
    literal::Literal,
};

pub struct Value(pub NodeValue<SpannedAst<Span>>, pub Span);

impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Brace) {
            let (value, span) = parse_spanned::<Struct>(input)?;
            Ok(Value(value.0, span))
        } else if input.peek(syn::token::Bracket) {
            let (value, span) = parse_spanned::<List>(input)?;
            Ok(Value(value.0, span))
        } else if input.peek(syn::token::Paren) {
            let (value, span) = parse_spanned::<Tuple>(input)?;
            Ok(Value(value.0, span))
        } else {
            let (value, span) = parse_spanned::<Literal>(input)?;
            Ok(Value(value.0, span))
        }
    }
}

fn parse_spanned<T: Parse>(input: ParseStream) -> syn::Result<(T, Span)> {
    let start_span = input.span();
    let value = input.parse::<T>()?;
    Ok((value, start_span.join(input.span()).unwrap_or(start_span)))
}
