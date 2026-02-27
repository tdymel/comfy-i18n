use comfy_i18n_generator::{
    components::{Field, Implementation, Initialization, Struct, UsePath},
    generator::{Path, RustGenerator, RustType, RustValue},
    shared::{NamePascalCase, NameSnakeCase, ToBasicTokenStream},
};
use quote::{ToTokens, quote};
use syn::{DeriveInput, Ident, parse::Parse};

pub struct ComfyI18n {
    pub name: Ident,
    pub variants: Vec<Variant>,
}

pub struct Variant {
    pub name: Ident,
    pub fallback: bool,
}

impl ComfyI18n {
    pub fn name_snake_case(&self) -> NameSnakeCase {
        NameSnakeCase::from(self.name.to_string())
    }

    pub fn name_pascal_case(&self) -> NamePascalCase {
        self.name_snake_case().to_pascal_case()
    }

    fn context_derives(&self) -> proc_macro2::TokenStream {
        let name = &self.name_pascal_case();

        let partial_eq_match_arms = self
            .variants
            .iter()
            .map(|variant| {
                format!(
                    "({0}::{1}, {0}::{1}) => true",
                    name,
                    variant.name.to_string()
                )
                .to_basic_token_stream()
            })
            .collect::<Vec<_>>();

        quote! {
            impl #name {
                pub const fn const_eq(&self, second: &Self) -> bool {
                    match (self, second) {
                        #(#partial_eq_match_arms),*,
                        _ => false,
                    }
                }
            }

            impl PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.const_eq(other)
                }
            }

            impl Eq for #name {}

            impl Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl Copy for #name {}
        }
    }

    fn context_impl(&self, context_name: NameSnakeCase) -> Implementation {
        let name = self.name_pascal_case();
        let get_context_match_arms = self
            .variants
            .iter()
            .map(|variant| {
                format!(
                    "{}::{} => {}",
                    name,
                    variant.name,
                    variant.name.to_string().to_uppercase()
                )
                .to_basic_token_stream()
            })
            .collect::<Vec<_>>();

        Implementation::new(
            Path::root().set_ty(context_name.to_pascal_case()),
            vec![quote! {
                pub const fn get(key: &#name) -> Self {
                    match key {
                        #(#get_context_match_arms),*
                    }
                }
            }],
        )
    }

    fn context_initializations(&self, context_name: NameSnakeCase) -> Vec<Initialization> {
        self.variants
            .iter()
            .map(|variant| {
                let path = Path::root().set_ty(context_name.to_pascal_case());
                Initialization::new_const(
                    path.clone(),
                    variant.name.to_string().into(),
                    RustValue::Struct {
                        path,
                        fields: vec![RustValue::Bool(variant.fallback)],
                    },
                )
            })
            .collect::<Vec<_>>()
    }
}

impl Parse for ComfyI18n {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input: DeriveInput = input.parse()?;
        let name = derive_input.ident;
        let variants = if let syn::Data::Enum(data_enum) = derive_input.data {
            data_enum
                .variants
                .into_iter()
                .map(|v| Variant {
                    name: v.ident,
                    fallback: v
                        .attrs
                        .iter()
                        .any(|attr| attr.meta.path().is_ident("fallback")),
                })
                .collect()
        } else {
            return Err(syn::Error::new_spanned(
                name,
                "ComfyI18n can only be derived for enums",
            ));
        };

        Ok(ComfyI18n { name, variants })
    }
}

impl ToTokens for ComfyI18n {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let context_name_snake_case =
            NameSnakeCase::from(format!("{}_context", self.name_snake_case()));
        let mut generator = RustGenerator::new(context_name_snake_case.clone());
        generator.add_root_content(UsePath::new(
            Path::root()
                .add_mod("super".into())
                .set_ty(self.name_pascal_case()),
        ));
        generator.add_root_content(Struct::new(
            context_name_snake_case.to_pascal_case(),
            vec![Field::public("fallback".into(), RustType::Bool)],
        ));
        generator.add_root_contents(self.context_initializations(context_name_snake_case.clone()));
        generator.add_root_content(self.context_impl(context_name_snake_case.clone()));
        generator.add_root_content(Implementation::new(
            Path::root().set_ty(self.name_pascal_case()),
            vec![
                {
                    let context_amount = self.variants.len();
                    quote! {
                        pub const fn amount() -> usize {
                            #context_amount
                        }
                    }
                },
                {
                    let context_name = context_name_snake_case.to_pascal_case();
                    quote! {
                        pub const fn context(&self) -> #context_name {
                            #context_name::get(self)
                        }
                    }
                },
                quote! {
                    pub const fn fallback(&self, available: [Option<Self>; Self::amount()]) -> Self {
                        let mut i = 0;
                        while i < available.len() {
                            if let Some(lang) = available[i]
                            {
                                if lang.const_eq(self) {
                                    return *self;
                                }
                            } else {
                                break;
                            }
                            i += 1;
                        }

                        i = 0;
                        while i < available.len() {
                            if let Some(lang) = available[i]
                            {
                                if lang.context().fallback {
                                    return lang;
                                }
                            } else {
                                break;
                            }
                            i += 1;
                        }

                        return available[0].expect("At least one language must be available.");
                    }
                }
            ],
        ));

        tokens.extend(self.context_derives());
        generator.to_tokens(tokens);
    }
}
