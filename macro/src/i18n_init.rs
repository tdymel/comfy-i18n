use comfy_i18n_generator::components::{Field, Implementation, Initialization, Struct, UsePath};
use comfy_i18n_generator::generator::{Path, RustGenerator, RustType, RustValue};
use comfy_i18n_generator::shared::{NamePascalCase, NameSnakeCase, ToBasicTokenStream};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Token};

pub struct I18nInit {
    variants: Vec<Variant>,
}

#[derive(Clone)]
pub struct Variant {
    pub name: Ident,
    pub fallback: bool,
}

impl Parse for I18nInit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variants = Punctuated::<Variant, Token![,]>::parse_terminated_with(input, |input| {
            let attributes = input.call(Attribute::parse_outer)?;
            let ident: Ident = input.parse()?;
            let fallback = attributes
                .iter()
                .any(|attr| attr.meta.path().is_ident("fallback"));
            Ok(Variant {
                name: ident,
                fallback,
            })
        })?;

        Ok(I18nInit {
            variants: variants.into_iter().collect(),
        })
    }
}

impl I18nInit {
    pub fn name_snake_case(&self) -> NameSnakeCase {
        NameSnakeCase::from("i18n")
    }

    pub fn name_pascal_case(&self) -> NamePascalCase {
        NamePascalCase::from("I18n".to_string())
    }

    fn impl_const_eq(&self) -> proc_macro2::TokenStream {
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

impl ToTokens for I18nInit {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let variants = self.variants.iter().map(|it| &it.name);
        tokens.extend(quote! {
            #[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
            pub enum I18n {
                #(#variants),*
            }
        });

        let context_name_snake_case =
            NameSnakeCase::from(format!("{}_context", self.name_snake_case()));
        let mut generator = RustGenerator::new(context_name_snake_case.clone());
        generator.add_root_content(UsePath::new(
            Path::root()
                .add_mod("super".into())
                .set_ty(self.name_pascal_case()),
        ));
        generator.add_root_content({
            let name = self.name_pascal_case();
            quote! {
                static _COMFY_I18N_COMPONENTS: std::sync::LazyLock<
                    std::sync::RwLock<
                        std::collections::HashMap<
                            &'static str,
                            Box<
                                dyn Fn(#name, std::collections::VecDeque<String>) -> &'static (dyn std::any::Any + Sync)
                                    + Sync
                                    + 'static
                                    + Send,
                            >,
                        >,
                    >,
                > = std::sync::LazyLock::new(|| std::sync::RwLock::new(std::collections::HashMap::new()));
            }
        });

        generator.add_root_content({
            let name = self.name_pascal_case();
            let default_variant = self.variants.iter().find(|it| it.fallback)
                .map(|it| it.name.to_string())
                .unwrap_or_else(|| self.variants.first().unwrap().name.to_string())
                .to_basic_token_stream();
            quote! {
                pub static _COMFY_I18N_DEFAULT_CONTEXT: std::sync::LazyLock<std::sync::RwLock<#name>> = 
                    std::sync::LazyLock::new(|| std::sync::RwLock::new(#name::#default_variant));

                #[macro_export]
                macro_rules! _comfy_i18n_default_context {
                    () => {
                        crate::#context_name_snake_case::_COMFY_I18N_DEFAULT_CONTEXT.read().unwrap()
                    }
                }

                #[macro_export]
                macro_rules! comfy_i18n_set_default_context {
                    ($context:expr) => {
                        {
                            let mut context = crate::#context_name_snake_case::_COMFY_I18N_DEFAULT_CONTEXT.write().unwrap();
                            *context = $context.clone();
                        }
                    }
                }

                impl #name {
                    pub fn set_default_context(context: Self) {
                        comfy_i18n_set_default_context!(context);
                    } 

                    pub fn set_as_default_context(&self) {
                        comfy_i18n_set_default_context!(self);
                    }
                }
            }
        });

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
                },
                {
                    let name = self.name_pascal_case();
                    quote! {
                        pub fn register_component(name: &'static str, callback: Box<
                                    dyn Fn(#name, std::collections::VecDeque<String>) -> &'static (dyn std::any::Any + Sync)
                                        + Sync
                                        + 'static
                                        + Send,
                                >) {
                            _COMFY_I18N_COMPONENTS.write().unwrap().insert(name, callback);
                        }
                    }
                },
                {
                    quote! {
                        fn _by_path<'a>(&'a self, mut path: std::collections::VecDeque<String>) -> &'static dyn std::any::Any {
                            if path.is_empty() {
                                panic!();
                            }
                            let key = path.pop_front().unwrap();

                            _COMFY_I18N_COMPONENTS
                                .read()
                                .unwrap()
                                .get(key.as_str())
                                .unwrap()(*self, path)
                        }

                        pub fn by_path<'a, T>(&'a self, path: &str) -> Option<&'static T> {
                            self._by_path(path.split(|c| c == '.' || c == '[')
                                .map(|segment| {
                                    if segment.ends_with(']') {
                                        segment[..segment.len() - 1].to_string()
                                    } else {
                                        segment.to_string()
                                    }
                                })
                                .collect())
                                .downcast_ref::<T>()
                        }
                    }
                }
            ],
        ));

        tokens.extend(self.impl_const_eq());
        generator.to_tokens(tokens);
    }
}
