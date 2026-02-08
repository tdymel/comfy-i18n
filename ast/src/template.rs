use std::fmt::Write;

use crate::Identifier;

#[derive(Debug, Clone)]
pub struct Template(pub Vec<Piece>);

impl Template {
    pub fn arguments(&self) -> Vec<(&ArgumentName, &Option<Specifier>)> {
        self.0
            .iter()
            .filter_map(|piece| match piece {
                Piece::Argument { name, specifier } => Some((name, specifier)),
                _ => None,
            })
            .collect()
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for piece in &self.0 {
            write!(f, "{}", piece)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Piece {
    Literal(String),
    BracketOpen,
    BracketClose,
    Argument {
        name: ArgumentName,
        specifier: Option<Specifier>,
    },
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Literal(literal) => f.write_str(literal),
            Piece::BracketOpen => f.write_str("{{"),
            Piece::BracketClose => f.write_str("}}"),
            Piece::Argument { name, specifier } => {
                f.write_char('{')?;
                write!(f, "{}", name)?;
                if let Some(specifier) = specifier {
                    f.write_char(':')?;
                    write!(f, "{}", specifier)?;
                }
                f.write_char('}')
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ArgumentName {
    Const(NameRef),
    ArgumentKey(ArgumentKey),
}

impl std::fmt::Display for ArgumentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentName::Const(name_ref) => write!(f, "{}", name_ref),
            ArgumentName::ArgumentKey(argument_key) => write!(f, "{}", argument_key),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NameRef {
    Ast {
        origin: AstRefOrigin,
        path: Vec<Identifier>,
    },
    Other(String),
}

impl std::fmt::Display for NameRef {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NameRef::Ast { origin: _, path: _ } => todo!(),
            NameRef::Other(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstRefOrigin {
    RootNode,
    SelfNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentKey {
    Index(usize),
    Name(String),
}

impl std::fmt::Display for ArgumentKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentKey::Index(index) => write!(f, "{}", index),
            ArgumentKey::Name(name) => f.write_str(name),
        }
    }
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

impl std::fmt::Display for Specifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fill_character)?;
        write!(f, "{}", self.alignment)?;
        if self.sign {
            f.write_char('+')?;
        }
        if self.alternate_form {
            f.write_char('#')?;
        }
        if self.pad_zero {
            f.write_char('0')?;
        }
        write!(f, "{}", self.width)?;
        write!(f, "{}", self.precision)?;
        write!(f, "{}", self.ty)?;

        Ok(())
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

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Binary => f.write_char('b'),
            Type::Octal => f.write_char('o'),
            Type::LowerHex => f.write_char('x'),
            Type::UpperHex => f.write_char('X'),
            Type::Pointer => f.write_char('p'),
            Type::LowerExp => f.write_char('e'),
            Type::UpperExp => f.write_char('E'),
            Type::Debug => f.write_char('?'),
            Type::Display => Ok(()),
        }
    }
}

/// Alignment variants of the specifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Auto,
}

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alignment::Left => f.write_char('<'),
            Alignment::Center => f.write_char('^'),
            Alignment::Right => f.write_char('>'),
            Alignment::Auto => Ok(()),
        }
    }
}

/// Width variants of the specifier.
#[derive(Debug, Clone)]
pub enum Width {
    Dynamic(ArgumentKey),
    Fixed(u16),
}

impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Width::Dynamic(argument_key) => {
                write!(f, "{}", argument_key)?;
                f.write_char('$')
            }
            Width::Fixed(amount) => write!(f, "{}", amount),
        }
    }
}

/// Precision variants of the specifier.
#[derive(Debug, Clone)]
pub enum Precision {
    Auto,
    Dynamic(ArgumentKey),
    Fixed(u16),
}

impl std::fmt::Display for Precision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Precision::Dynamic(argument_key) => {
                f.write_char('.')?;
                write!(f, "{}", argument_key)?;
                f.write_char('$')
            }
            Precision::Fixed(amount) => {
                f.write_char('.')?;
                write!(f, "{}", amount)
            }
            Precision::Auto => Ok(()),
        }
    }
}
