use std::str::FromStr;

use comfy_i18n_ast::{NodeValue, SpannedAst};
use proc_macro2::Span;

use crate::{
    Parser,
    comfy::{
        composite::{List, Struct, Tuple},
        field::{Field, Fields},
        literal::Literal,
        value::Value,
    },
    error::Error,
};

mod composite;
mod field;
mod literal;
mod value;

impl Parser for proc_macro2::TokenStream {
    fn parse_fields(self) -> Result<Vec<SpannedAst<Span>>, Error> {
        syn::parse2::<Fields>(self)
            .map(|it| it.0)
            .map_err(Error::from)
    }

    fn parse_field(self) -> Result<SpannedAst<Span>, Error> {
        syn::parse2::<Field>(self)
            .map(|it| it.0)
            .map_err(Error::from)
    }

    fn parse_node_value(self) -> Result<NodeValue<SpannedAst<Span>>, Error> {
        syn::parse2::<Value>(self)
            .map(|it| it.0)
            .map_err(Error::from)
    }

    fn parse_literal(self) -> Result<NodeValue<SpannedAst<Span>>, Error> {
        syn::parse2::<Literal>(self)
            .map(|it| it.0)
            .map_err(Error::from)
    }

    fn parse_struct(self) -> Result<NodeValue<SpannedAst<Span>>, Error> {
        syn::parse2::<Struct>(self)
            .map(|it| it.0)
            .map_err(Error::from)
    }

    fn parse_list(self) -> Result<NodeValue<SpannedAst<Span>>, Error> {
        syn::parse2::<List>(self)
            .map(|it| it.0)
            .map_err(Error::from)
    }

    fn parse_tuple(self) -> Result<NodeValue<SpannedAst<Span>>, Error> {
        syn::parse2::<Tuple>(self)
            .map(|it| it.0)
            .map_err(Error::from)
    }
}

impl Parser for &str {
    fn parse_fields(self) -> Result<Vec<SpannedAst<Span>>, Error> {
        proc_macro2::TokenStream::from_str(self)
            .unwrap()
            .parse_fields()
    }

    fn parse_field(self) -> Result<SpannedAst<Span>, Error> {
        proc_macro2::TokenStream::from_str(self)
            .unwrap()
            .parse_field()
    }

    fn parse_node_value(self) -> Result<NodeValue<SpannedAst<Span>>, Error> {
        proc_macro2::TokenStream::from_str(self)
            .unwrap()
            .parse_node_value()
    }

    fn parse_literal(self) -> Result<NodeValue<SpannedAst<Span>>, Error> {
        proc_macro2::TokenStream::from_str(self)
            .unwrap()
            .parse_literal()
    }

    fn parse_struct(self) -> Result<NodeValue<SpannedAst<Span>>, Error> {
        proc_macro2::TokenStream::from_str(self)
            .unwrap()
            .parse_struct()
    }

    fn parse_list(self) -> Result<NodeValue<SpannedAst<Span>>, Error> {
        proc_macro2::TokenStream::from_str(self)
            .unwrap()
            .parse_list()
    }

    fn parse_tuple(self) -> Result<NodeValue<SpannedAst<Span>>, Error> {
        proc_macro2::TokenStream::from_str(self)
            .unwrap()
            .parse_tuple()
    }
}
