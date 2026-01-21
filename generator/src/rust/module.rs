use quote::{ToTokens, quote};

use crate::rust::{
    initialization::Initialization, name::NameSnakeCase, strct::Struct, utils::ToBasicTokenSreamVec,
};

#[derive(Debug)]
pub struct Module {
    pub name: NameSnakeCase,
    pub strct: Option<Struct>,
    pub initializations: Vec<Initialization>,
    pub modules: Vec<Module>,
}

impl Module {
    pub fn new(name: NameSnakeCase) -> Self {
        Module {
            name,
            strct: Option::default(),
            initializations: Vec::default(),
            modules: Vec::default(),
        }
    }

    fn is_empty(&self) -> bool {
        self.strct.as_ref().map(|it| it.is_empty()).unwrap_or(false)
            && self.initializations.is_empty()
            && self.modules.iter().all(|module| module.is_empty())
    }
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.is_empty() {
            return;
        }

        let name = self.name.to_lowercase().to_token_stream();
        let strct = self.strct.to_token_stream();
        let initializations = self.initializations.to_token_stream();
        let modules = self.modules.to_token_stream();

        tokens.extend(quote! {
           pub mod #name {
                #strct

                #(#initializations)*

                #(#modules)*
           }
        });
    }
}
