use std::collections::VecDeque;

use quote::ToTokens;

use crate::shared::{NamePascalCase, NameSnakeCase, ToBasicTokenStream};

#[derive(Debug, Clone)]
pub struct Path {
    mods: VecDeque<NameSnakeCase>,
    ty: Option<NamePascalCase>,
}

impl Path {
    pub fn root() -> Self {
        Self {
            mods: VecDeque::new(),
            ty: None,
        }
    }

    pub fn add_mod(mut self, part: NameSnakeCase) -> Self {
        self.mods.push_back(part);
        self
    }

    pub fn set_ty(mut self, ty: NamePascalCase) -> Self {
        self.ty = Some(ty);
        self
    }

    pub fn has_no_mods(&self) -> bool {
        self.mods.is_empty()
    }

    pub fn pop_front(&mut self) -> Option<NameSnakeCase> {
        self.mods.pop_front()
    }

    pub fn iter_mods(&self) -> impl Iterator<Item = &NameSnakeCase> {
        self.mods.iter()
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.to_string().to_basic_token_stream())
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .mods
                .iter()
                .map(|it| it.to_string())
                .collect::<Vec<_>>()
                .join("::"),
        )?;

        if let Some(ty) = self.ty.as_ref() {
            if !self.mods.is_empty() {
                f.write_str("::")?;
            }
            f.write_str(&ty.to_string())?;
        }

        Ok(())
    }
}

impl From<comfy_i18n_ast::Path> for Path {
    fn from(value: comfy_i18n_ast::Path) -> Self {
        value
            .iter()
            .enumerate()
            .fold(Path::root(), |acc, (index, segment)| {
                let part = match segment {
                    comfy_i18n_ast::Identifier::Field(field) => field.to_string(),
                    comfy_i18n_ast::Identifier::Element(index) => format!("elem{}", index),
                };

                let mut acc = acc.add_mod(part.clone().into());
                if index == value.len() - 1 {
                    acc = acc.set_ty(NameSnakeCase::from(part).to_pascal_case());
                }

                acc
            })
    }
}
