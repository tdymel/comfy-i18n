mod error;
mod comfy;

use comfy_i18n_ast::{NodeValue, SpannedAst};
use proc_macro2::Span;

use crate::error::Error;

pub trait Parser {
    fn parse_fields(self) -> Result<Vec<SpannedAst<Span>>, Error>;
    fn parse_field(self) -> Result<SpannedAst<Span>, Error>;
    fn parse_node_value(self) -> Result<NodeValue<SpannedAst<Span>>, Error>;
    fn parse_literal(self) -> Result<NodeValue<SpannedAst<Span>>, Error>;
    fn parse_struct(self) -> Result<NodeValue<SpannedAst<Span>>, Error>;
    fn parse_list(self) -> Result<NodeValue<SpannedAst<Span>>, Error>;
    fn parse_tuple(self) -> Result<NodeValue<SpannedAst<Span>>, Error>;
}
