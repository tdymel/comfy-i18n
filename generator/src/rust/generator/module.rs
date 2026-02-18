use std::collections::HashMap;

use quote::{ToTokens, quote};

use crate::{generator::Path, rust::shared::NameSnakeCase};

pub struct Module {
    name: NameSnakeCase,
    content: Vec<Box<dyn ToTokens>>,
    modules: HashMap<NameSnakeCase, Self>,
}

impl Module {
    pub fn new(name: NameSnakeCase) -> Self {
        Self {
            name,
            content: Vec::new(),
            modules: HashMap::new(),
        }
    }

    pub fn add_content<Content: ToTokens + 'static>(&mut self, content: Content) {
        self.content.push(Box::new(content));
    }

    pub fn by_path(&mut self, mut path: Path) -> &mut Self {
        if path.has_no_mods() {
            return self;
        }

        let name = path.pop_front().unwrap();

        if !self.modules.contains_key(&name) {
            self.modules.insert(name.clone(), Module::new(name.clone()));
        }

        self.modules.get_mut(&name).unwrap().by_path(path)
    }
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mod_name = &self.name;
        let content = &self.content;
        let sub_modules = self.modules.values();

        tokens.extend(quote! {
            pub mod #mod_name {
                #(#content)*

                #(#sub_modules)*
            }
        });
    }
}
