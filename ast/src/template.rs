use crate::{Identifier, LiteralValue, NodeId};

#[derive(Debug, Clone)]
pub struct Template(pub Vec<Piece>);

#[derive(Debug, Clone)]
pub enum Piece {
    Literal(String),
    Argument {
        name: ArgumentName,
        specififer: Option<Specifier>,
    },
}

#[derive(Debug, Clone)]
pub enum ArgumentName {
    Const(NameRef),
    Function { name: NameRef, args: Vec<FnArg> },
    ArgumentKey(ArgumentKey),
}

#[derive(Debug, Clone)]
pub enum NameRef {
    Ast { node_id: NodeId, field: Identifier },
    Other(String),
}

#[derive(Debug, Clone)]
pub enum FnArg {
    Literal(LiteralValue),
    Function { name: NameRef, args: Vec<FnArg> },
    Const(NameRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentKey {
    Index(usize),
    Name(String),
}

#[derive(Debug, Clone)]
pub struct Specifier {
    pub ty: Type,
    pub alternate_form: bool,
    pub fill_character: char,
    pub alignment: Alignment,
    pub sign: bool,
    pub pad_zero: bool,
    pub width: Width,
    pub precision: Precision,
}

impl Default for Specifier {
    fn default() -> Self {
        Self {
            ty: Type::Display,
            alternate_form: false,
            fill_character: ' ',
            alignment: Alignment::Auto,
            sign: false,
            pad_zero: false,
            width: Width::Fixed(0),
            precision: Precision::Auto,
        }
    }
}

/// Type variants of the specifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Binary,
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    LowerExp,
    UpperExp,
    Debug,
    Display,
}

/// Alignment variants of the specifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Auto,
}

/// Width variants of the specifier.
#[derive(Debug, Clone)]
pub enum Width {
    Dynamic(ArgumentKey),
    Fixed(u16),
}

/// Precision variants of the specifier.
#[derive(Debug, Clone)]
pub enum Precision {
    Auto,
    Dynamic(ArgumentKey),
    Fixed(u16),
}
