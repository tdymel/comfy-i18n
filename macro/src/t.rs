use comfy_i18n_generator::shared::ToBasicTokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, Ident, LitInt, LitStr, Path, Token,
    parse::{Parse, ParseStream, discouraged::Speculative},
};

pub struct T {
    translation_key: KeyKind,
    context: Option<Expr>,
    ty: Option<Path>,
    args: Vec<KeyValuePair>,
}

enum KeyKind {
    Static(LitStr),
    Dynamic(Expr),
}

impl Parse for T {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut context = None;
        let mut ty = None;
        let mut args = Vec::new();

        // Parse the first key/value pair for the translation_key
        let first_key_value: KeyValuePair = input.parse()?;
        let translation_key = match first_key_value.value {
            Expr::Lit(expr_lit) => {
                if let syn::Lit::Str(lit_str) = expr_lit.lit {
                    KeyKind::Static(lit_str)
                } else {
                    return Err(input.error("Translation key must be a str literal or a variable."));
                }
            }
            _ => KeyKind::Dynamic(first_key_value.value),
        };

        // Parse remaining key/value pairs
        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            let kv: KeyValuePair = input.parse()?;
            match kv.key {
                Some(key) if key == "context" => {
                    context = Some(kv.value);
                }
                Some(key) if key == "ty" => {
                    if let Expr::Path(expr_path) = kv.value {
                        ty = Some(expr_path.path);
                    } else {
                        return Err(input.error("Expected a path for `ty` key"));
                    }
                }
                _ => {
                    args.push(kv);
                }
            }
        }

        Ok(T {
            translation_key,
            context,
            ty,
            args,
        })
    }
}

#[derive(Clone)]
struct KeyValuePair {
    pub key: Option<String>,
    pub value: Expr,
}

impl Parse for KeyValuePair {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        let mut key = if fork.peek(LitStr) {
            Some(fork.parse::<LitStr>()?.value())
        } else if fork.peek(Ident) {
            Some(fork.parse::<Ident>()?.to_string())
        } else if fork.peek(LitInt) {
            Some(fork.parse::<LitInt>()?.base10_digits().to_string())
        } else {
            None
        };

        let has_key = if fork.peek(Token![=]) && fork.peek2(Token![>]) {
            fork.parse::<Token![=]>()?;
            fork.parse::<Token![>]>()?;
            true
        } else if fork.peek(Token![=]) {
            fork.parse::<Token![=]>()?;
            true
        } else if fork.peek(Token![:]) {
            fork.parse::<Token![:]>()?;
            true
        } else {
            false
        };

        if has_key {
            input.advance_to(&fork);
        } else {
            key = None;
        }

        let value: Expr = input.parse()?;

        Ok(KeyValuePair { key, value })
    }
}

impl ToTokens for T {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let context = if let Some(context) = &self.context {
            quote! { #context }
        } else {
            quote! { _comfy_i18n_default_context!() }
        };

        let mut counted_index = 0usize;
        let mut args = self.args.clone();
        args.sort_by(|l, r| l.key.cmp(&r.key));

        let args = args
            .into_iter()
            .map(|kv| {
                let value = &kv.value;
                // TODO: I need to know what kind of type it is... => We probably have to use some sort of struct
                quote! { &#value }
            })
            .collect::<Vec<_>>();
        let fn_suffix = if args.is_empty() {
            quote! {}
        } else {
            quote! { .format(#(#args),*) }
        };

        if let KeyKind::Static(key) = &self.translation_key {
            let str_key = key.value();
            let new_path = str_key
                .split(".")
                .map(|part| format!("{}()", part))
                .collect::<Vec<_>>()
                .join(".");
            let new_path = if args.is_empty() {
                new_path.to_basic_token_stream()
            } else {
                format!(
                    "{}_value(){}",
                    &new_path[..new_path.len() - 2],
                    fn_suffix.to_string()
                )
                .to_basic_token_stream()
            };

            tokens.extend(quote! { #context.#new_path })
        } else if let KeyKind::Dynamic(dynamic_expr) = &self.translation_key {
            let ty = if let Some(ty) = &self.ty {
                quote! { #ty }
            } else {
                quote! { &'static str }
            };

            tokens.extend(quote! { #context.by_path::<#ty>(&#dynamic_expr)
                .expect("Translation key does not exist or the provided type is incorrect.")#fn_suffix });
        }
    }
}
