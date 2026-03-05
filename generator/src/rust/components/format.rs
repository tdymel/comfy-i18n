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
    pub parent_struct: Path,
    pub root_name: NameSnakeCase,
}

impl Format {
    pub fn new(
        name: NamePascalCase,
        context_key: Path,
        template: Template,
        parent_struct: Path,
        root_name: NameSnakeCase,
    ) -> Self {
        Self {
            name,
            context_key,
            template,
            parent_struct,
            root_name,
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

        let self_access_path = self
            .parent_struct
            .clone()
            .prepend_mod(self.root_name.clone())
            .clear_ty()
            .to_access_path();
        let self_args = self
            .template
            .arguments()
            .iter()
            .flat_map(|(arg, _)| match arg {
                ArgumentName::Const(NameRef::Ast { origin, path, .. }) => {
                    let arg_name = arg.to_string().replace(".", "_");
                    let access_path = match origin {
                        AstRefOrigin::RootNode => path
                            .iter()
                            .fold(format!("self.comfy_i18n_context.{}()", self.root_name), |acc, it| {
                                format!("{}.{}()", acc, NameSnakeCase::from(it.clone()))
                            }),
                        AstRefOrigin::SelfNode => path
                            .iter()
                            .fold(format!("self.comfy_i18n_context.{}", self_access_path), |acc, it| {
                                format!("{}.{}()", acc, NameSnakeCase::from(it.clone()))
                            }),
                    };
                    Some(
                        // TODO: Correct ArgumentType
                        format!("let {0} = {1}; args.add_argument_value_unchecked(\"{0}\", comfy_i18n::macro_use::ArgumentValue::Display(&{0}));", arg_name, access_path)
                            .to_basic_token_stream(),
                    )
                },
                _ => None,
            })
            .collect::<Vec<_>>();

        let parent_struct_path = &self.parent_struct.ty();
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
                comfy_i18n_context: #context_key,
                template: comfy_i18n::macro_use::Template
            }

            impl #format_name {
                pub fn new(comfy_i18n_context: #context_key, template: comfy_i18n::macro_use::Template) -> Self {
                    Self { comfy_i18n_context, template }
                }
                // TODO: Handle const arguments
                // TODO: Add i18n root keyword: i18n.DE.component.tree.path
                // TODO: Add context root keyword: context.component.tree.path
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

            impl super::#parent_struct_path {
                // TODO: Visibility
                pub fn #value_fn_name(&'static self) -> &'static #format_name {
                    // TODO: Fallback!
                    self.#field_name.as_ref().unwrap()
                }

                pub fn #field_name(&'static self #(, #format_arg_names: #format_arg_types)*) -> String {
                    self.#value_fn_name()(#(#format_arg_names),*)
                }
            }
        });
    }
}
