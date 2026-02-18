use quote::{ToTokens, quote};

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Const,
    Static,
}

impl ToTokens for VariableType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            VariableType::Const => quote! { const },
            VariableType::Static => quote! { static },
        });
    }
}
