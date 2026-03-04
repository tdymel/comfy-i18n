use quote::{ToTokens, quote};

use crate::rust_generator::Path;

pub struct UsePath(Path);

impl UsePath {
    pub fn new(path: Path) -> Self {
        Self(path)
    }
}

impl ToTokens for UsePath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = &self.0;

        tokens.extend(quote! {
            use #path;
        });
    }
}
