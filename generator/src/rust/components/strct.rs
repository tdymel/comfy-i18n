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
        let field_names = self
            .fields
            .iter()
            .map(|it| it.name.clone())
            .collect::<Vec<_>>();

        let new_tys = self
            .fields
            .iter()
            .map(|it| Field {
                name: it.name.clone(),
                optional: it.optional,
                ty: it.ty.clone(),
                public: false,
            })
            .collect::<Vec<_>>();

        // TODO: Not every type will support copy! Validation!
        tokens.extend(quote! {
            #[derive(Clone, Copy)]
            pub struct #name {
                #(#fields),*
            }

            impl #name {
                pub const fn new(
                    #(#new_tys),*
                ) -> Self {
                    Self {
                        #(#field_names),*
                    }
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: NameSnakeCase,
    pub ty: RustType,
    pub optional: bool,
    pub public: bool,
}

impl Field {
    pub fn optional(name: NameSnakeCase, ty: RustType) -> Self {
        Self {
            name,
            ty,
            optional: true,
            public: false,
        }
    }

    pub fn new(name: NameSnakeCase, ty: RustType) -> Self {
        Self {
            name,
            ty,
            optional: false,
            public: false,
        }
    }

    pub fn public(name: NameSnakeCase, ty: RustType) -> Self {
        Self {
            name,
            ty,
            optional: false,
            public: true,
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name.to_lowercase();
        let type_name = &self.ty;

        if self.optional {
            tokens.extend(quote! {
                #name: Option<#type_name>
            });
        } else {
            if self.public {
                tokens.extend(quote! {
                    pub #name: #type_name
                });
            } else {
                tokens.extend(quote! {
                    #name: #type_name
                });
            }
        }
    }
}
