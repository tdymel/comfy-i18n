use quote::{ToTokens, quote};

use crate::generator::Path;

pub struct Implementation {
    path: Path,
    functions: Vec<Box<dyn ToTokens + 'static>>,
}

impl Implementation {
    pub fn new<Content: ToTokens + 'static>(path: Path, functions: Vec<Content>) -> Self {
        Self {
            path,
            functions: functions
                .into_iter()
                .map(|function| Box::new(function) as Box<dyn ToTokens>)
                .collect(),
        }
    }
}

impl ToTokens for Implementation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = &self.path;
        let functions = &self.functions;

        tokens.extend(quote! {
            impl #path {
                #(#functions)*
            }
        })
    }
}
