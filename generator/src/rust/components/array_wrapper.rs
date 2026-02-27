use quote::{ToTokens, quote};

use crate::generator::{Path, RustType};

pub struct ArrayWrapper {
    absolute_path: Path,
    context_path: Path,
    ty: RustType,
    size: usize,
}

impl ArrayWrapper {
    pub const fn new(absolute_path: Path, context_path: Path, ty: RustType, size: usize) -> Self {
        Self {
            absolute_path,
            context_path,
            ty,
            size,
        }
    }
}

// TODO: Do we even need this wrapper now? Is there a case where the content is a wrapped type?
impl ToTokens for ArrayWrapper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.absolute_path.ty().unwrap();
        let context_path = &self.context_path;
        let ty = &self.ty;
        let size = &self.size;

        tokens.extend(quote! {
            #[derive(Clone)]
            pub struct #name {
                value: [#ty; #size]
            }

            impl #name {
                pub fn new(_comfy_i18n_context: #context_path, value: [#ty; #size]) -> Self {
                    Self { value }
                }

                pub fn value(&self) -> [#ty; #size] {
                    self.value
                }
            }

            impl core::ops::Index<usize> for #name {
                type Output = #ty;

                fn index(&self, index: usize) -> &Self::Output {
                    &self.value[index]
                }
            }
        });
    }
}
