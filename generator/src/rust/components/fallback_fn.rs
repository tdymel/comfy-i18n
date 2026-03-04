use quote::quote;

use crate::{
    rust_generator::{Path, RustType},
    shared::{NameSnakeCase, ToBasicTokenStream},
};

pub fn fallback_fn(
    field_name: &NameSnakeCase,
    ty: &RustType,
    context_key: &Path,
    access_path: proc_macro2::TokenStream,
    access_suffix: proc_macro2::TokenStream,
    available_context_variants: &[String],
) -> proc_macro2::TokenStream {
    let contexts = available_context_variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            format!("contexts[{}] = Some({}::{});", index, context_key, variant)
                .to_basic_token_stream()
        })
        .collect::<Vec<_>>();

    quote! {
        pub fn #field_name(&'static self) -> &'static #ty {
            let mut contexts = [None; #context_key::amount()];
            #(#contexts)*
            if contexts.contains(&Some(self.comfy_i18n_context)) {
                return self.#access_suffix;
            }

            self.comfy_i18n_context.fallback(contexts).#access_path.#access_suffix
        }
    }
}
