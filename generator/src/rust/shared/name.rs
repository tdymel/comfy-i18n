use comfy_i18n_ast::Identifier;
use quote::ToTokens;

use crate::rust::shared::ToBasicTokenStream;

#[derive(Debug, Clone)]
pub struct NamePascalCase(String);

impl NamePascalCase {
    pub fn to_snake_case(&self) -> NameSnakeCase {
        let mut snake_case = String::new();

        let mut encountered_lower_case = false;
        for ch in self.0.chars() {
            if ch.is_uppercase() && encountered_lower_case {
                snake_case.push('_');
                encountered_lower_case = false;
            } else if ch.is_lowercase() {
                encountered_lower_case = true;
            }
            snake_case.push(ch);
        }

        snake_case.to_lowercase().into()
    }
}

impl From<Identifier> for NamePascalCase {
    fn from(value: Identifier) -> Self {
        NameSnakeCase::from(value).to_pascal_case()
    }
}

impl From<String> for NamePascalCase {
    fn from(value: String) -> Self {
        if value.contains("_") {
            return NameSnakeCase::from(value).to_pascal_case();
        }
        NamePascalCase(value)
    }
}

impl ToTokens for NamePascalCase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.to_basic_token_stream());
    }
}

impl std::fmt::Display for NamePascalCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameSnakeCase(String);

impl NameSnakeCase {
    pub fn tuple_index(index: usize) -> Self {
        format!("tuple_index{}", index).into()
    }

    pub fn to_lowercase(&self) -> Self {
        Self(self.0.to_lowercase())
    }

    pub fn to_uppercase(&self) -> Self {
        Self(self.0.to_uppercase())
    }

    pub fn to_pascal_case(&self) -> NamePascalCase {
        self.0
            .split('_')
            .map(|part| {
                let mut chars = part.chars();

                match chars.next() {
                    Some(first) => format!(
                        "{}{}",
                        first.to_uppercase(),
                        chars.collect::<String>().to_lowercase()
                    ),

                    None => String::new(),
                }
            })
            .collect::<String>()
            .into()
    }

    pub fn concat(self, other: NameSnakeCase) -> Self {
        (self.0 + "_" + other.0.as_str()).into()
    }

    pub fn last_part(&self) -> String {
        self.0.split("_").last().unwrap().to_string()
    }
}

impl From<String> for NameSnakeCase {
    fn from(value: String) -> Self {
        if !value.contains("_")
            && value
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_uppercase())
        {
            return NamePascalCase::from(value).to_snake_case();
        }
        Self(value.to_lowercase())
    }
}

impl From<&str> for NameSnakeCase {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<Identifier> for NameSnakeCase {
    fn from(value: Identifier) -> Self {
        match value {
            Identifier::Field(name) => name,
            Identifier::TupleIndex(index) => format!("tuple_index{}", index),
            Identifier::ArrayIndex(index) => format!("array_index{}", index),
        }
        .into()
    }
}

impl ToTokens for NameSnakeCase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.to_basic_token_stream());
    }
}

impl std::fmt::Display for NameSnakeCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
