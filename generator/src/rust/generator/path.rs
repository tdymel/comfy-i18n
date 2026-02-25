use std::collections::VecDeque;

use comfy_i18n_ast::Identifier;
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

    pub fn prepend_mod(mut self, part: NameSnakeCase) -> Self {
        self.mods.push_front(part);
        self
    }

    pub fn add_mod(mut self, part: NameSnakeCase) -> Self {
        self.mods.push_back(part);
        self
    }

    pub fn ty(&self) -> Option<&NamePascalCase> {
        self.ty.as_ref()
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

    pub fn len_mods(&self) -> usize {
        self.mods.len()
    }

    pub fn last_mod(&self) -> Option<&NameSnakeCase> {
        self.mods.iter().last()
    }

    pub fn relative_to(&self, other_path: &Self) -> Self {
        let mut new_path = self.clone();
        for (p1, p2) in other_path.iter_mods().zip(self.iter_mods()) {
            if p1 == p2 {
                new_path.pop_front();
            }
        }
        new_path
    }

    pub fn to_access_path(&self) -> String {
        self.iter_mods()
            .enumerate()
            .fold(String::new(), |acc, (index, segment)| {
                let result = if segment.to_string().starts_with("tuple_index") {
                    format!("{}.{}()", acc, &segment.to_string()[11..])
                } else if segment.to_string().starts_with("array_index") {
                    format!("{}[{}]", acc, &segment.to_string()[11..])
                } else {
                    format!("{}.{}()", acc, segment)
                };

                if index == 0 {
                    result[1..].to_string()
                } else {
                    result
                }
            })
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
                    Identifier::Field(field) => field.to_string(),
                    Identifier::TupleIndex(index) => format!("tuple_index{}", index),
                    Identifier::ArrayIndex(_) => "array_index0".to_string(),
                };

                let mut acc = acc.add_mod(part.clone().into());
                if index == value.len() - 1 {
                    acc = acc.set_ty(NameSnakeCase::from(part).to_pascal_case());
                }

                acc
            })
    }
}
