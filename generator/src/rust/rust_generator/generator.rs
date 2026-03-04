use quote::ToTokens;

use crate::{
    rust::{rust_generator::module::Module, shared::NameSnakeCase},
    rust_generator::Path,
};

pub struct RustGenerator {
    root: Module,
}

impl RustGenerator {
    pub fn new(root_name: NameSnakeCase) -> Self {
        Self {
            root: Module::new(root_name),
        }
    }

    pub fn add_content<Content: ToTokens + 'static>(&mut self, path: Path, content: Content) {
        let module = if path.has_no_mods() {
            &mut self.root
        } else {
            self.root.by_path(path)
        };

        module.add_content(content);
    }

    pub fn add_root_content<Content: ToTokens + 'static>(&mut self, content: Content) {
        self.add_content(Path::root(), content)
    }

    pub fn add_contents<Content: ToTokens + 'static>(
        &mut self,
        path: Path,
        contents: Vec<Content>,
    ) {
        contents
            .into_iter()
            .for_each(|content| self.add_content(path.clone(), content))
    }

    pub fn add_root_contents<Content: ToTokens + 'static>(&mut self, contents: Vec<Content>) {
        self.add_contents(Path::root(), contents)
    }
}

impl ToTokens for RustGenerator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.root.to_tokens(tokens);
    }
}
