use comfy_i18n_ast::{Ast, CompositeValue, Identifier, LiteralValue, NodeValue, StringValue};
use comfy_i18n_generator::{
    components::{Format, Implementation, Initialization, array_wrapper, strct, tuple_wrapper},
    rust_generator::{Context, Path, RustGenerator, RustValue},
    shared::{NameSnakeCase, ToBasicTokenStream},
};
use comfy_i18n_parser::Parser;
use quote::{ToTokens, quote};
use syn::{Ident, parse::Parse};

pub struct I18n {
    pub name: Ident,
    pub context: Context,
}

impl Parse for I18n {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<syn::token::Comma>()?;

        let localizations = input
            .parse::<proc_macro2::TokenStream>()?
            .parse_fields()
            .unwrap()
            .into_iter()
            .map(Ast::from)
            .collect();

        let context = Context::new(localizations, name.to_string().into());

        Ok(Self { name, context })
    }
}

impl I18n {
    pub fn name_snake_case(&self) -> NameSnakeCase {
        self.name.to_string().into()
    }

    fn context_impl(&self) -> Implementation {
        let name = self.name_snake_case();
        let strct_name = name.to_pascal_case();

        let available_contexts = self
            .context
            .iter()
            .enumerate()
            .map(|(index, ctx)| {
                format!(
                    "contexts[{}] = Some(Self::{});",
                    index,
                    ctx.identifier.to_string().to_uppercase()
                )
                .to_basic_token_stream()
            })
            .collect::<Vec<_>>();

        let match_arms = self
            .context
            .iter()
            .map(|ctx| {
                format!(
                    "Self::{} => &{}",
                    ctx.identifier,
                    ctx.identifier.to_string().to_uppercase()
                )
                .to_basic_token_stream()
            })
            .collect::<Vec<_>>();
        Implementation::new(
            self.context.context_key().clone(),
            vec![quote! {
                pub fn #name(&self) -> &'static #strct_name {
                    let mut contexts = [None; Self::amount()];
                    #(#available_contexts)*

                    match self.fallback(contexts) {
                        #(#match_arms),*,
                        _ => unreachable!(),
                    }
                }
            }],
        )
    }

    fn component_init(&self) -> proc_macro2::TokenStream {
        let name = self.name_snake_case();
        let name_str = name.to_string();
        let context_key = self.context.context_key();

        quote! {
            use comfy_i18n::macro_use::ctor;

            #[ctor]
            fn _comfy_i18n_init_test() {
                #context_key::register_component(#name_str,
                    Box::new(|context: #context_key, path: std::collections::VecDeque<String>| context.#name().by_path(path)))
            }
        }
    }
}

impl ToTokens for I18n {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut generator = RustGenerator::new(self.name_snake_case());
        let reference_tree = self.context.reference_tree();

        generator.add_root_content(self.context_impl());
        generator.add_root_content(self.component_init());

        for variant in self.context.context_variants() {
            generator.add_root_content(Initialization::new_static(
                Path::root().set_ty(self.name_snake_case().to_pascal_case()),
                variant.clone().into(),
                RustValue::by_variant(reference_tree, &self.context, &variant),
            ));
        }

        for node in reference_tree
            .traverse()
            .filter(|node| {
                matches!(
                    &node.value,
                    NodeValue::Composite { .. }
                        | NodeValue::Literal(LiteralValue::String(StringValue::Template(_)))
                )
            })
            .filter(|node| {
                if let Identifier::ArrayIndex(index) = node.identifier
                    && index > 0
                {
                    false
                } else {
                    true
                }
            })
        {
            let path = self.context.relative_path_to_root(&node.id);
            match &node.value {
                NodeValue::Composite {
                    children,
                    value: CompositeValue::Struct,
                } => {
                    generator.add_content(path.clone(), strct(node, children, &self.context, true))
                }
                NodeValue::Composite {
                    children,
                    value: CompositeValue::Tuple,
                } => generator.add_content(path, tuple_wrapper(node, children, &self.context)),
                NodeValue::Composite {
                    children,
                    value: CompositeValue::List { .. },
                } => generator.add_content(path, array_wrapper(node, children, &self.context)),
                NodeValue::Literal(LiteralValue::String(StringValue::Template(template))) => {
                    generator.add_content(
                        path,
                        Format::new(
                            node.identifier.clone().into(),
                            self.context.context_key().clone(),
                            template.clone(),
                            self.context.relative_path_to_root(&node.parent.unwrap()),
                            self.context.root_name(),
                        ),
                    );
                }
                _ => unreachable!(),
            }
        }

        generator.to_tokens(tokens);
    }
}
