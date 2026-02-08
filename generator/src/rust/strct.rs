use comfy_i18n_ast::{ArgumentKey, ArgumentName, Template};
use quote::{ToTokens, quote};

use crate::rust::{
    name::{NamePascalCase, NameSnakeCase},
    utils::{ToBasicTokenSreamVec, ToBasicTokenStream},
};

#[derive(Debug)]
pub enum StructVariation {
    Format(Format),
    Struct(Struct),
}

impl StructVariation {
    pub fn rename_strct_name(&mut self, name: NameSnakeCase) {
        match self {
            StructVariation::Format(format) => format.rename_ref_strct(name.to_pascal_case()),
            StructVariation::Struct(strct) => strct.rename(name.to_pascal_case()),
        }
    }
}

impl ToTokens for StructVariation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            StructVariation::Format(fmt) => fmt.to_tokens(tokens),
            StructVariation::Struct(strct) => strct.to_tokens(tokens),
        }
    }
}

#[derive(Debug)]
pub struct Format {
    pub name: NamePascalCase,
    pub template: Template,
    pub ref_strct: Option<NamePascalCase>,
}

impl Format {
    pub fn rename_ref_strct(&mut self, name: NamePascalCase) {
        self.ref_strct = Some(name);
    }
}

impl ToTokens for Format {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let format_name = self.name.to_token_stream();
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
                            dfmt::dformat_unchecked!(self.template, #(#dformat_args),*)
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
                });

        if let Some(ref_strct) = &self.ref_strct {
            // TODO: Assuming name was changed to the field name
            let field_name = self.name.to_snake_case().to_token_stream();
            let ref_strct = ref_strct.to_token_stream();
            tokens.extend(quote! {
                impl #ref_strct {
                    pub fn #field_name(&self, #(#format_fn_args),*) -> String {
                        (self.#field_name)(#(#format_arg_names),*)
                    }
                }
            });
        }
    }
}

#[derive(Debug)]
pub struct Struct {
    pub name: NamePascalCase,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: NamePascalCase) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    pub fn with_fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = fields;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn rename(&mut self, name: NamePascalCase) {
        self.name = name;
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.is_empty() {
            return;
        }

        let name = self.name.to_token_stream();
        let fields = self.fields.to_token_stream();

        // TODO: Not every type will support copy! Validation!
        tokens.extend(quote! {
            #[derive(Clone, Copy)]
            pub struct #name {
                #(#fields),*
            }
        });
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: NameSnakeCase,
    pub ty: RustType,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.name.to_lowercase().to_token_stream();
        let type_name = self.ty.to_token_stream();

        tokens.extend(quote! {
            pub #name: #type_name
        });
    }
}

#[derive(Debug)]
pub enum RustType {
    String,
    Format {
        name: NamePascalCase,
    },
    Char,
    Bool,
    Float {
        bits: u8,
    },
    Integer {
        unsigned: bool,
        bits: u8,
    },
    Tuple(Vec<RustType>),
    List {
        ty: Box<RustType>,
        amount: Option<usize>,
    },
    Struct {
        mod_names: Vec<NameSnakeCase>,
        name: NamePascalCase,
    },
    Other {
        mod_names: Vec<NameSnakeCase>,
        name: NamePascalCase,
    },
}

impl ToTokens for RustType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            RustType::String => quote! { &'static str },
            RustType::Format { name } => {
                quote! { #name }
            }
            RustType::Char => quote! { char },
            RustType::Bool => quote! { bool },
            RustType::Float { bits } => match bits {
                32 | 64 => {
                    let type_name = format!("f{}", bits).to_basic_token_stream();
                    quote! { #type_name }
                }
                _ => panic!(),
            },
            RustType::Integer { unsigned, bits } => match bits {
                8 | 16 | 32 | 64 | 128 => {
                    let type_name = match unsigned {
                        true => format!("u{}", bits),
                        false => format!("i{}", bits),
                    }
                    .to_basic_token_stream();
                    quote! { #type_name }
                }
                _ => panic!(),
            },
            RustType::Tuple(rust_types) => {
                let type_names = rust_types.to_token_stream();
                quote! {
                    (#(#type_names),*)
                }
            }
            RustType::List {
                ty: rust_type,
                amount,
            } => {
                let type_name = rust_type.to_token_stream();
                if let Some(amount) = amount {
                    quote! { [#type_name; #amount]}
                } else {
                    quote! {Vec<#type_name>}
                }
            }
            RustType::Struct { name, mod_names } | RustType::Other { name, mod_names } => {
                let type_name = name.to_token_stream();

                if mod_names.is_empty() {
                    quote! { #type_name }
                } else {
                    let mod_names = mod_names.to_token_stream();
                    quote! { #(#mod_names)::* :: #type_name }
                }
            }
        });
    }
}
