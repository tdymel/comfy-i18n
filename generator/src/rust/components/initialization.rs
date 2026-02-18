use quote::{ToTokens, quote};

use crate::{
    generator::Path,
    rust::{
        generator::{RustValue, VariableType},
        shared::NameSnakeCase,
    },
};

#[derive(Debug)]
pub struct Initialization {
    pub var_ty: VariableType,
    pub ty: Path,
    pub name: NameSnakeCase,
    pub value: RustValue,
}

impl Initialization {
    pub fn new(var_ty: VariableType, ty: Path, name: NameSnakeCase, value: RustValue) -> Self {
        Self {
            var_ty,
            ty,
            name,
            value,
        }
    }

    pub fn new_const(ty: Path, name: NameSnakeCase, value: RustValue) -> Self {
        Self::new(VariableType::Const, ty, name, value)
    }
}

impl ToTokens for Initialization {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let var_ty = &self.var_ty;
        let ty = &self.ty;
        let name = self.name.to_uppercase();
        let value = &self.value;

        if self.var_ty == VariableType::Static {
            tokens.extend(quote! {
                pub #var_ty #name: std::sync::LazyLock<#ty> = std::sync::LazyLock::new(|| #value);
            });
        } else {
            tokens.extend(quote! {
                pub #var_ty #name: #ty = #value;
            });
        }
    }
}
