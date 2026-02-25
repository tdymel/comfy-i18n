use comfy_i18n_ast::{Ast, CompositeValue, Identifier, LiteralValue, NodeValue, SpannedAst, StringValue};
use comfy_i18n_generator::{
    components::{Field, Format, Implementation, Initialization, Struct, TupleWrapper, ValueWrapper},
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
        let reference_tree = self.context.reference_tree();

        generator.add_root_content(self.context_impl());

        for variant in self.context.context_variants() {
            generator.add_root_content(Initialization::new_const(
                Path::root().set_ty(self.name_snake_case().to_pascal_case()),
                variant.clone().into(),
                RustValue::by_variant(reference_tree, &self.context, &variant),
            ));
        }

        for node in reference_tree.traverse() {
            let path = self.context.relative_path_to_root(&node.id);
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

                    let mut pairs = children.iter().collect::<Vec<_>>();
                    pairs.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
                    let mut fields = pairs
                        .iter()
                        .map(|(_, field)| {
                            Field::optional(
                                field.identifier.clone().into(),
                                RustType::new(field, &self.context, &path),
                            )
                        })
                        .collect::<Vec<_>>();

                    // TODO: This fallback impl was implmented at multiple places already
                    // TODO: Refactor this mess
                    let context_key = self.context.context_key().clone();
                    let absolute_path = path.clone().prepend_mod(self.name_snake_case()).set_ty(node.identifier.clone().into());
                    let access_path = absolute_path.to_access_path().to_basic_token_stream();

                    let fns = children
                    .values()
                    .filter(|ast| !matches!(ast.value, NodeValue::Literal(LiteralValue::String(StringValue::Template(..)))))
                    .map(|ast| {
                        let name = NameSnakeCase::from(ast.identifier.clone());
                        let contexts = self.context.available_context_variants(&ast.id)
                            .enumerate()
                            .map(|(index, variant)| 
                                format!("contexts[{}] = Some({}::{});", index, context_key, variant).to_basic_token_stream()
                            );

                        let ty = RustType::new(ast, &self.context, &path);
                        quote! {
                            pub fn #name(&self) -> #ty {
                                let mut contexts = [None; #context_key::amount()];
                                #(#contexts)*
                                self.comfy_i18n_context.fallback(contexts).#access_path.#name.unwrap()
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                    generator.add_content(
                        path.clone(),
                        Implementation::new(Path::root().set_ty(strct_name.clone()), fns),
                    );

                    // TODO: Push this deeper into the struct?
                    fields.push(Field::new(
                        "comfy_i18n_context".into(),
                        RustType::Other(self.context.context_key().clone()),
                    ));
                    generator.add_content(path.clone(), Struct::new(strct_name.clone(), fields));
                }
                NodeValue::Composite { children, value: CompositeValue::Tuple } => {
                    if let Identifier::ArrayIndex(index) = node.identifier 
                        && index > 0 
                        {
                            continue;
                        } 

                    let mut pairs = children.iter().collect::<Vec<_>>();
                    pairs.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
                    let tys = pairs
                        .iter()
                        .map(|(_, field)| 
                                RustType::new(field, &self.context, &path)
                        )
                        .collect::<Vec<_>>();

                    for (index, ty) in tys.iter().enumerate() {
                        generator.add_content(path.clone(), ValueWrapper::new(
                            path.clone().prepend_mod(self.name_snake_case()).add_mod(format!("tuple_index{}", index).into()).set_ty(format!("Elem{}", index).into()), 
                        self.context.context_key().clone(), 
                        self.context.context_variants().map(|it| it.to_string()).collect(), 
                        ty.clone()
                        ));
                    }

                    generator.add_content(path.clone(), TupleWrapper::new(
                        path.prepend_mod(self.name_snake_case()).set_ty(node.identifier.clone().into()), 
                        self.context.context_key().clone(), 
                        tys));
                }
                NodeValue::Literal(LiteralValue::String(
                    StringValue::Template(template),
                )) => {
                    generator.add_content(
                        path,
                        Format::new(
                            node.identifier.clone().into(),
                            template.clone(),
                            self.context.relative_path_to_root(&node.parent.unwrap()),
                        ),
                    );
                }
                _ => {}
            }
        }

        generator.to_tokens(tokens);
    }
}
