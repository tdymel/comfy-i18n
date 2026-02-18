use comfy_i18n_ast::{Ast, CompositeValue, NodeValue, SpannedAst};
use comfy_i18n_generator::{
    components::{Field, Format, Implementation, Initialization, Struct},
    generator::{Context, Path, RustGenerator, RustType, RustValue},
    shared::{NamePascalCase, NameSnakeCase, ToBasicTokenStream},
};
use comfy_i18n_parser::Parser;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Ident, parse::Parse};

pub struct I18n {
    pub name: Ident,
    pub translations: SpannedAst<Span>,
    pub context: Context,
}

impl Parse for I18n {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name_key = input.parse::<Ident>()?;
        assert_eq!(name_key.to_string().as_str(), "name");
        input.parse::<syn::token::Colon>()?;
        let name_value = input.parse::<Ident>()?;
        input.parse::<syn::token::Comma>()?;

        let key_key = input.parse::<Ident>()?;
        assert_eq!(key_key.to_string().as_str(), "key");
        input.parse::<syn::token::Colon>()?;
        let key_value = input.parse::<syn::Path>()?;
        input.parse::<syn::token::Comma>()?;

        // This is everything but robust!
        let translations = input
            .parse::<proc_macro2::TokenStream>()?
            .parse_field()
            .unwrap();

        let context_key =
            key_value
                .segments
                .iter()
                .enumerate()
                .fold(Path::root(), |acc, (index, segment)| {
                    if index == key_value.segments.len() - 1 {
                        acc.set_ty(segment.ident.to_string().into())
                    } else {
                        acc.add_mod(segment.ident.to_string().into())
                    }
                });

        let context = Context::new(
            Ast::from(translations.clone()),
            NamePascalCase::from(name_value.to_string()).to_snake_case(),
            context_key,
        );

        Ok(Self {
            name: name_value,
            translations,
            context,
        })
    }
}

impl I18n {
    pub fn name_snake_case(&self) -> NameSnakeCase {
        NamePascalCase::from(self.name.to_string()).to_snake_case()
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
                    "Self::{} => {}",
                    ctx.identifier,
                    ctx.identifier.to_string().to_uppercase()
                )
                .to_basic_token_stream()
            })
            .collect::<Vec<_>>();
        Implementation::new(
            self.context.context_key().clone(),
            vec![quote! {
                pub const fn #name(&self) -> #strct_name {
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
}

impl ToTokens for I18n {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut generator = RustGenerator::new(self.name_snake_case());
        let main_context = self.context.main();

        generator.add_root_content(self.context_impl());

        for localization in self.context.iter() {
            generator.add_root_content(Initialization::new_const(
                Path::root().set_ty(self.name_snake_case().to_pascal_case()),
                localization.identifier.clone().into(),
                RustValue::new(localization, &self.context),
            ));
        }

        for node in main_context.traverse() {
            let path = self.context.relative_path(&node.id);
            match &node.value {
                NodeValue::Composite {
                    children,
                    value: CompositeValue::Struct,
                } => {
                    let strct_name = if path.has_no_mods() {
                        self.name_snake_case().to_pascal_case()
                    } else {
                        node.identifier.clone().into()
                    };

                    let mut fields = children
                        .values()
                        .map(|field| {
                            Field::optional(
                                field.identifier.clone().into(),
                                RustType::new(field, &self.context),
                            )
                        })
                        .collect::<Vec<_>>();

                    // TODO: This fallback impl was implmented at multiple places already
                    // TODO: Refactor this mess
                    // TODO: Comfy_context shouldnt be optional
                    if path.has_no_mods() {
                        let context_key = self.context.context_key().clone();
                        let root_name = self.name_snake_case();
                        let relative_path = self
                            .context
                            .relative_path(&node.id)
                            .iter_mods()
                            .map(|it| {
                                if it.to_string().starts_with("elem") {
                                    it.to_string()[4..].to_string().to_basic_token_stream()
                                } else {
                                    it.to_string().to_basic_token_stream()
                                }
                            })
                            .collect::<Vec<_>>();

                        let fns = fields
                        .iter()
                        .filter(|field| !matches!(field.ty, RustType::Format(..)))
                        .map(|field| {
                            let name = &field.name;
                            let ty = &field.ty;

                            quote! {
                                pub const fn #name(&self) -> #ty {
                                    let mut contexts = [None; #context_key::amount()];
                                    contexts[0] = Some(#context_key::DE);
                                    contexts[1] = Some(#context_key::EN);
                                    self.comfyi18n_context.fallback(contexts).#root_name().#name.unwrap()
                                }
                            }
                        })
                        .collect::<Vec<_>>();

                        generator.add_content(
                            path.clone(),
                            Implementation::new(Path::root().set_ty(strct_name.clone()), fns),
                        );
                    }

                    // TODO: Push this deeper into the struct?
                    fields.push(Field::new(
                        "comfyi18n_context".into(),
                        RustType::Other(self.context.context_key().clone()),
                    ));
                    generator.add_content(path.clone(), Struct::new(strct_name.clone(), fields));
                }
                NodeValue::Literal(comfy_i18n_ast::LiteralValue::String(
                    comfy_i18n_ast::StringValue::Template(template),
                )) => {
                    generator.add_content(
                        path,
                        Format::new(
                            node.identifier.clone().into(),
                            template.clone(),
                            self.context.relative_path(&node.parent.unwrap()),
                        ),
                    );
                }
                _ => {}
            }
        }

        generator.to_tokens(tokens);
    }
}
