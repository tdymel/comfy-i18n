use quote::{ToTokens, quote};

use crate::rust::{
    generator::RustType,
    shared::{NamePascalCase, NameSnakeCase},
};

#[derive(Debug)]
pub struct Struct {
    pub name: NamePascalCase,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: NamePascalCase, fields: Vec<Field>) -> Self {
        Self { name, fields }
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.is_empty() {
            return;
        }

        let name = &self.name;
        let fields = &self.fields;

        // TODO: Not every type will support copy! Validation!
        tokens.extend(quote! {
            #[derive(Clone, Copy)]
            pub struct #name {
                #(#fields),*
            }
        });
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: NameSnakeCase,
    pub ty: RustType,
    pub optional: bool,
}

impl Field {
    pub fn optional(name: NameSnakeCase, ty: RustType) -> Self {
        Self {
            name,
            ty,
            optional: true,
        }
    }

    pub fn new(name: NameSnakeCase, ty: RustType) -> Self {
        Self {
            name,
            ty,
            optional: false,
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name.to_lowercase();
        let type_name = &self.ty;

        // TODO: Make private
        if self.optional {
            tokens.extend(quote! {
                pub #name: Option<#type_name>
            });
        } else {
            tokens.extend(quote! {
                pub #name: #type_name
            });
        }
    }
}
