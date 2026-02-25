use comfy_i18n_ast::{ArgumentKey, ArgumentName, Template};
use quote::{ToTokens, quote};

use crate::{
    generator::Path,
    rust::shared::{NamePascalCase, ToBasicTokenStream},
};

#[derive(Debug)]
pub struct Format {
    pub name: NamePascalCase,
    pub template: Template,
    pub parent_struct: Path,
}

impl Format {
    pub fn new(name: NamePascalCase, template: Template, parent_struct: Path) -> Self {
        Self {
            name,
            template,
            parent_struct,
        }
    }
}

impl ToTokens for Format {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let format_name = &self.name;
        // TODO: Order for stability?
        let non_const_arguments = self
            .template
            .arguments()
            .iter()
            .filter_map(|(argument_name, specifier)| match argument_name {
                ArgumentName::ArgumentKey(key) => Some((key, specifier.as_ref())),
                _ => None,
            })
            .collect::<Vec<_>>();

        let format_fn_args = non_const_arguments
            .iter()
            .map(|(name, specifier)| {
                // Handle index args
                let name = format!("arg_{}", name).to_basic_token_stream();
                let ty = format!(
                    "&dyn core::fmt::{:?}",
                    specifier
                        .map(|it| it.ty)
                        .unwrap_or(comfy_i18n_ast::Type::Display)
                )
                .to_basic_token_stream();

                quote! {
                    #name : #ty
                }
            })
            .collect::<Vec<_>>();

        let dformat_args = non_const_arguments.iter().map(|(name, ..)| {
            match name {
                ArgumentKey::Index(index) => format!("arg_{}", index),
                ArgumentKey::Name(name) => format!("{0} = arg_{0}", name),
            }
            .to_basic_token_stream()
        });

        let format_arg_names = non_const_arguments
            .iter()
            .map(|(name, ..)| format!("arg_{}", name).to_basic_token_stream())
            .collect::<Vec<_>>();
        let format_arg_types = non_const_arguments.iter().map(|(_, specifier)| {
            format!(
                "&dyn core::fmt::{:?}",
                specifier
                    .map(|it| it.ty)
                    .unwrap_or(comfy_i18n_ast::Type::Display)
            )
            .to_basic_token_stream()
        });

        let parent_struct_path = &self.parent_struct;
        let field_name = self.name.to_snake_case();

        tokens.extend(quote! {
                    #[derive(Clone, Copy)]
                    pub struct #format_name {
                        pub template: &'static str
                    }

                    impl #format_name {
                        // TODO: Handle const arguments
                        // TODO: Create new static template after const arguments
                        // TODO: Handle no args case 
                        pub fn format(&self, #(#format_fn_args),*) -> String {
                            comfy_i18n::macro_use::dformat_unchecked!(self.template, #(#dformat_args),*)
                        }
                    }

                    impl core::ops::Deref for #format_name {
                        type Target = dyn Fn(#(#format_arg_types),*) -> String;

                        fn deref(&self) -> &Self::Target {
                            let uninit_callable: Self = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                            let uninit_closure = move |#(#format_fn_args),*| {
                                Self::format(&uninit_callable, #(#format_arg_names),*)
                            };
                            let size_of_closure = core::mem::size_of_val(&uninit_closure);
                            fn second<'a, T>(_a: &T, b: &'a T) -> &'a T {
                                b
                            }
                            let reference_to_closure = second(&uninit_closure, unsafe { core::mem::transmute(self) });
                            core::mem::forget(uninit_closure);
                            assert_eq!(size_of_closure, core::mem::size_of::<Self>());
                            let reference_to_trait_object = reference_to_closure as &Self::Target;
                            reference_to_trait_object
                        }
                    }

                    impl super::#parent_struct_path {
                        pub fn #field_name(&self, #(#format_fn_args),*) -> String {
                            (self.#field_name.unwrap())(#(#format_arg_names),*)
                        }
                    }
                });
    }
}
