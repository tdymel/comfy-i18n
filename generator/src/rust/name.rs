use comfy_i18n_ast::Identifier;
use quote::ToTokens;

use crate::rust::utils::ToBasicTokenStream;

#[derive(Debug, Clone)]
pub struct NamePascalCase(String);

impl NamePascalCase {
    pub fn to_snake_case(&self) -> NameSnakeCase {
        let mut snake_case = String::new();

        for (i, ch) in self.0.chars().enumerate() {
            if ch.is_uppercase() {
                if i != 0 {
                    snake_case.push('_');
                }
            }
            snake_case.push(ch);
        }

        snake_case.to_lowercase().into()
    }
}

impl From<String> for NamePascalCase {
    fn from(value: String) -> Self {
        NamePascalCase(value)
    }
}

impl ToTokens for NamePascalCase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.to_basic_token_stream());
    }
}

#[derive(Debug, Clone)]
pub struct NameSnakeCase(String);

impl NameSnakeCase {
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
}

impl From<String> for NameSnakeCase {
    fn from(value: String) -> Self {
        // TODO: More logic to detect other cases and convert them on the fly
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
            Identifier::Element(index) => format!("elem{}", index),
        }
        .into()
    }
}

impl ToTokens for NameSnakeCase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.to_basic_token_stream());
    }
}
