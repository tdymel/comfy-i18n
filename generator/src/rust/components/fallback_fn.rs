use quote::quote;

use crate::{
    components::to_fmt_args,
    rust_generator::{Path, RustType},
    shared::{NameSnakeCase, ToBasicTokenStream},
};

pub fn fallback_fn(
    public: bool,
    field_name: &NameSnakeCase,
    function_name: Option<NameSnakeCase>,
    ty: &RustType,
    context_key: &Path,
    access_path: proc_macro2::TokenStream,
    access_suffix: proc_macro2::TokenStream,
    available_context_variants: &[String],
    convert_format_fn: bool
) -> proc_macro2::TokenStream {
    let contexts = available_context_variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            format!("contexts[{}] = Some({}::{});", index, context_key, variant)
                .to_basic_token_stream()
        })
        .collect::<Vec<_>>();

    let fn_name = function_name.unwrap_or(field_name.clone());
    let pub_mod = if public {
        quote! { pub }
    } else {
        quote! {}
    };

    let mut fmt_args = quote! {}; 
    let mut access_suffix = access_suffix;
    let mut return_ty = quote! { &'static #ty };
    if convert_format_fn && let RustType::Format(_, template) = ty {
        let (format_arg_names, format_arg_types) = to_fmt_args(template);
        fmt_args = quote! { #(, #format_arg_names: #format_arg_types)* };
        access_suffix = quote! { #access_suffix(#(#format_arg_names),*) };
        return_ty = quote! { String };
    };

    quote! {
        #pub_mod fn #fn_name(&'static self #fmt_args) -> #return_ty {
            let mut contexts = [None; #context_key::amount()];
            #(#contexts)*
            if contexts.contains(&Some(self.context)) {
                return self.#access_suffix;
            }

            self.context.fallback(contexts).#access_path.#access_suffix
        }
    }
}
