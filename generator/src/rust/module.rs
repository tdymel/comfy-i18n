use quote::{ToTokens, quote};

use crate::rust::{
    initialization::Initialization, name::NameSnakeCase, strct::StructVariation,
    utils::ToBasicTokenSreamVec,
};

#[derive(Debug)]
pub struct Module {
    pub name: NameSnakeCase,
    pub strcts: Vec<StructVariation>,
    pub initializations: Vec<Initialization>,
    pub modules: Vec<Module>,
}

impl Module {
    pub fn new(name: NameSnakeCase) -> Self {
        Module {
            name,
            strcts: Vec::default(),
            initializations: Vec::default(),
            modules: Vec::default(),
        }
    }

    fn is_empty(&self) -> bool {
        self.strcts.is_empty()
            && self.initializations.is_empty()
            && self.modules.iter().all(|module| module.is_empty())
    }

    // Very special method to just rename the module, its initializations etc.
    pub fn rename(&mut self, name: NameSnakeCase) {
        self.name = name.clone();

        self.strcts
            .iter_mut()
            .for_each(|it| it.rename_strct_name(name.clone()));

        self.initializations
            .iter_mut()
            .for_each(|it| it.rename(name.clone()));
    }
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.is_empty() {
            return;
        }

        let name = self.name.to_lowercase().to_token_stream();
        let strct = self.strcts.to_token_stream();
        let initializations = self.initializations.to_token_stream();
        let modules = self.modules.to_token_stream();

        tokens.extend(quote! {
           pub mod #name {
                #(#strct)*

                #(#initializations)*

                #(#modules)*
           }
        });
    }
}
