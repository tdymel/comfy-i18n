use std::collections::HashMap;

use comfy_i18n_ast::{CompositeValue, Identifier, LiteralValue, NodeValue, SpannedAst};
use proc_macro2::Span;
use syn::{
    Error, braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::comfy::{field::Fields, literal::Literal, value::Value};

pub struct Struct(pub NodeValue<SpannedAst<Span>>);

impl Parse for Struct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let fields = content.parse::<Fields>()?;
        Ok(Struct(NodeValue::Composite {
            children: HashMap::from(fields),
            value: CompositeValue::Struct,
        }))
    }
}

pub struct Tuple(pub NodeValue<SpannedAst<Span>>);

impl Parse for Tuple {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let values = Punctuated::<Value, syn::token::Comma>::parse_terminated(&content)?
            .into_iter()
            .enumerate()
            .map(|(index, field)| SpannedAst::new(Identifier::TupleIndex(index), field.1, field.0))
            .collect::<Vec<_>>();

        Ok(Tuple(NodeValue::Composite {
            children: HashMap::from(Fields(values)),
            value: CompositeValue::Tuple,
        }))
    }
}

pub struct List(pub NodeValue<SpannedAst<Span>>);

impl Parse for List {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);
        let values = Punctuated::<Value, syn::token::Comma>::parse_separated_nonempty(&content)?
            .into_iter()
            .enumerate()
            .map(|(index, field)| SpannedAst::new(Identifier::ArrayIndex(index), field.1, field.0))
            .collect::<Vec<_>>();

        let amount = if content.peek(syn::token::Semi) {
            content.parse::<syn::token::Semi>()?;
            if let Literal(NodeValue::Literal(LiteralValue::Integer(amount))) =
                content.parse::<Literal>()?
            {
                amount.to_usize().ok_or_else(|| {
                    Error::new(
                        content.span(),
                        "comfy-i18n-parser: List repetition amount is not usize.",
                    )
                })
            } else {
                Err(Error::new(
                    content.span(),
                    "comfy-i18n-parser: List repetition amount is not an integer.",
                ))
            }
        } else {
            Ok(values.len())
        }?;

        Ok(List(NodeValue::Composite {
            children: HashMap::from(Fields(values)),
            value: CompositeValue::List { amount },
        }))
    }
}
