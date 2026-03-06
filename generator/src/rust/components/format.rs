use comfy_i18n_ast::{ArgumentKey, ArgumentName, AstRefOrigin, NameRef, Template};
use quote::{ToTokens, quote};

use crate::{
    components::hackfn,
    rust::shared::{NamePascalCase, ToBasicTokenStream},
    rust_generator::Path,
    shared::NameSnakeCase,
};

#[derive(Debug)]
pub struct Format {
    pub name: NamePascalCase,
    pub context_key: Path,
    pub template: Template,
    pub parent_path: Path,
    pub root_name: NameSnakeCase,
    pub is_parent_struct: bool
}

impl Format {
    pub fn new(
        name: NamePascalCase,
        context_key: Path,
        template: Template,
        parent_path: Path,
        root_name: NameSnakeCase,
        is_parent_struct: bool
    ) -> Self {
        Self {
            name,
            context_key,
            template,
            parent_path,
            root_name,
            is_parent_struct
        }
    }
}

impl ToTokens for Format {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let format_name = &self.name;
        let mut non_const_arguments = self
            .template
            .arguments()
            .iter()
            .filter_map(|(argument_name, specifier)| match argument_name {
                ArgumentName::ArgumentKey(key) => Some((key, specifier.as_ref())),
                _ => None,
            })
            .collect::<Vec<_>>();
        non_const_arguments.sort_by(|l, r| l.0.cmp(r.0));

        let format_arg_names = non_const_arguments
            .iter()
            .map(|(name, ..)| format!("arg_{}", name).to_basic_token_stream())
            .collect::<Vec<_>>();
        let format_arg_types = non_const_arguments
            .iter()
            .map(|(_, specifier)| {
                format!(
                    "&dyn core::fmt::{:?}",
                    specifier
                        .map(|it| it.ty)
                        .unwrap_or(comfy_i18n_ast::Type::Display)
                )
                .to_basic_token_stream()
            })
            .collect::<Vec<_>>();

        let dfmt_arg_names = non_const_arguments
            .iter()
            .map(|(name, _)| match name {
                ArgumentKey::Index(index) => index.to_string().to_basic_token_stream(),
                ArgumentKey::Name(name) => name.to_token_stream(),
            })
            .collect::<Vec<_>>();

        let dfmt_arg_types = non_const_arguments
            .iter()
            .map(|(_, specifier)| {
                format!(
                    "{:?}",
                    specifier
                        .map(|it| it.ty)
                        .unwrap_or(comfy_i18n_ast::Type::Display)
                )
                .to_basic_token_stream()
            })
            .collect::<Vec<_>>();

        let self_args = self
            .template
            .arguments()
            .iter()
            .flat_map(|(arg, specifier)| match arg {
                ArgumentName::Const(NameRef::Ast { origin, path, .. }) => {
                    let arg_name = arg.to_string().replace(".", "_");
                    let access_path = match origin {
                        AstRefOrigin::RootNode => path
                            .iter()
                            .fold(format!("self.context.{}()", self.root_name), |acc, it| {
                                format!("{}.{}()", acc, NameSnakeCase::from(it.clone()))
                            }),
                        AstRefOrigin::SelfNode => {
                            let access_path = self
                                .parent_path
                                .clone()
                                .prepend_mod(self.root_name.clone())
                                .clear_ty()
                                .to_access_path();
                            path
                            .iter()
                            .fold(format!("self.context.{}", access_path), |acc, it| {
                                format!("{}.{}()", acc, NameSnakeCase::from(it.clone()))
                            })
                        },
                        AstRefOrigin::ContextNode => {
                            let access_path = self
                                .parent_path
                                .clone()
                                .clear_ty()
                                .to_access_path();
                            path
                            .iter()
                            .fold(format!("self.context.{}", access_path), |acc, it| {
                                format!("{}.{}()", acc, NameSnakeCase::from(it.clone()))
                            })
                        },
                        AstRefOrigin::I18nNode => {
                            let mut access_gen_path = self
                                .parent_path
                                .clone();
                            let context = access_gen_path.pop_front().unwrap(); 
                            let access_path = access_gen_path
                                .clone()
                                .clear_ty()
                                .to_access_path();

                            path
                            .iter()
                            .fold(format!("crate::I18n::{}.{}", context.to_uppercase(), access_path), |acc, it| {
                                format!("{}.{}()", acc, NameSnakeCase::from(it.clone()))
                            })
                        },
                    };

                    let specifier_type = specifier
                        .as_ref()
                        .map(|it| it.ty)
                        .unwrap_or(comfy_i18n_ast::Type::Display);
                    Some(
                        format!("let {0} = {1}; args.add_argument_value_unchecked(\"{0}\", comfy_i18n::macro_use::ArgumentValue::{2:?}(&{0}));", arg_name, access_path, specifier_type)
                            .to_basic_token_stream(),
                    )
                },
                _ => None,
            })
            .collect::<Vec<_>>();

        let parent_struct_path = &self.parent_path.ty();
        let field_name = self.name.to_snake_case();
        let context_key = &self.context_key;

        let hackfn = hackfn(
            format_name,
            &"format".into(),
            &format_arg_types,
            &format_arg_names,
            "String".to_basic_token_stream(),
        );

        let value_fn_name = format!("{}_value", field_name).to_basic_token_stream();

        tokens.extend(quote! {
            #[derive(Clone)]
            pub struct #format_name {
                context: #context_key,
                template: comfy_i18n::macro_use::Template
            }

            impl #format_name {
                pub fn new(context: #context_key, template: comfy_i18n::macro_use::Template) -> Self {
                    Self { context, template }
                }
                // TODO: Handle const arguments
                // TODO: Apparently specifying these arguments here in the 
                // place has a singificant performance overhead for the formatting.
                // We are in a unique position where we know which arguments we want to put in there, so we could 
                // Preprocess the template and develop a special interface to shortcut this.
                pub fn format(&self #(, #format_arg_names: #format_arg_types)*) -> String {
                    let mut args = self.template.arguments();
                    #(#self_args)*
                    #(args.add_argument_value_unchecked(#dfmt_arg_names, comfy_i18n::macro_use::ArgumentValue::#dfmt_arg_types(#format_arg_names));)*
                    args.format().unwrap()
                }
            }

            #hackfn
        });

        if self.is_parent_struct {
            tokens.extend(quote! {
                impl super::#parent_struct_path {
                    pub fn #field_name(&'static self #(, #format_arg_names: #format_arg_types)*) -> String {
                        self.#value_fn_name()(#(#format_arg_names),*)
                    }
                }
            });
        }
    }
}
