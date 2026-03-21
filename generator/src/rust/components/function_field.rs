use comfy_i18n_ast::{Ast, CompositeValue, LiteralValue, NodeValue};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    components::hackfn,
    rust_generator::Context,
    shared::{NamePascalCase, NameSnakeCase, ToBasicTokenStream},
};

pub fn function_field(node: &Ast, context: &Context) -> TokenStream {
    let (args, ret_ty) = if let NodeValue::Literal(lit_value) = &node.value
        && let LiteralValue::Function { args, ret_ty, .. } = lit_value
    {
        (args, ret_ty)
    } else {
        panic!()
    };

    let strct_name = NamePascalCase::from(node.identifier.clone());
    let method_name = NameSnakeCase::from(node.identifier.clone());
    let arg_types = args
        .iter()
        .map(|(_, ty)| ty.to_basic_token_stream())
        .collect();
    let arg_names = args
        .iter()
        .map(|(name, _)| name.to_basic_token_stream())
        .collect();
    let ret_ty = ret_ty.to_basic_token_stream();

    let hackfn = hackfn(
        &strct_name,
        &method_name,
        &arg_types,
        &arg_names,
        ret_ty.clone(),
    );

    // TODO: Use body from all variants
    let method_bodies = context
        .context_variants()
        .flat_map(|it| {
            context
                .get_variant(&node.id, it.clone())
                .map(|ast| (it, ast))
        })
        .flat_map(|(variant, ast)| match &ast.value {
            NodeValue::Literal(LiteralValue::Function { body, .. }) => Some((variant, body)),
            _ => None,
        })
        .map(|(variant, body)| {
            format!("crate::I18n::{} => {}", variant, body).to_basic_token_stream()
        })
        .collect::<Vec<_>>();

    let parent_path = context.relative_path_to_root(&node.parent.unwrap());
    let parent_is_struct = matches!(
        context.get(&node.parent.unwrap()).value,
        NodeValue::Composite {
            value: CompositeValue::Struct,
            ..
        }
    );

    let method_name_value = format!("{}_value", method_name).to_basic_token_stream();
    let parent_impl = if parent_is_struct {
        quote! {
            impl super::#parent_path {
                pub fn #method_name(&'static self #(,#arg_names: #arg_types)*) -> #ret_ty {
                    self.#method_name_value()(#(#arg_names),*)
                }
            }
        }
    } else {
        quote! {}
    };
    quote! {
        #[derive(Clone, Copy)]
        pub struct #strct_name {
            context: crate::I18n
        }

        impl #strct_name {
            pub const fn new(context: crate::I18n) -> Self {
                Self {
                    context
                }
            }

            pub fn #method_name(&self #(,#arg_names: #arg_types)*) -> #ret_ty {
                match self.context {
                    #(#method_bodies,)*
                    _ => unreachable!()
                }
            }
        }

        #hackfn

        #parent_impl
    }
}
