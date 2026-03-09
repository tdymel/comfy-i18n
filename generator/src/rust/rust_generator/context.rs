use std::collections::HashMap;

use comfy_i18n_ast::{
    Ast, CompositeValue, Identifier, LiteralValue, NodeId, NodeValue, StringValue,
};

use crate::{rust_generator::Path, shared::NameSnakeCase};

pub struct Context {
    root_name: NameSnakeCase,
    localizations: Vec<Ast>,
    id_to_path: HashMap<NodeId, comfy_i18n_ast::Path>,
    path_to_id: HashMap<comfy_i18n_ast::Path, NodeId>,
    relative_path_to_is_copy: HashMap<comfy_i18n_ast::Path, bool>,
    reference_tree: Ast,
}

impl Context {
    pub fn new(localizations: Vec<Ast>, root_name: NameSnakeCase) -> Self {
        let id_to_path = localizations
            .iter()
            .fold(HashMap::new(), |mut acc, context| {
                let map = context.id_to_path_map();
                acc.extend(map);
                acc
            });

        let path_to_id = localizations
            .iter()
            .fold(HashMap::new(), |mut acc, context| {
                let map = context.path_to_node_id_map();
                acc.extend(map);
                acc
            });

        let mut ref_locals = localizations.clone();
        let mut reference_tree = ref_locals.remove(0);
        for ast in ref_locals {
            reference_tree.merge(ast);
        }

        let mut relative_path_to_is_copy = HashMap::new();
        create_is_copy_tree(
            &reference_tree,
            &id_to_path,
            &localizations,
            &mut relative_path_to_is_copy,
        );

        Self {
            root_name,
            localizations,
            id_to_path,
            path_to_id,
            relative_path_to_is_copy,
            reference_tree,
        }
    }

    pub fn relative_path_to_root(&self, node_id: &NodeId) -> Path {
        let path: Path = self
            .id_to_path
            .get(node_id)
            .unwrap()
            .clone()
            .remove(0)
            .unwrap()
            .into();

        if path.has_no_mods() {
            path.set_ty(self.root_name.to_pascal_case())
        } else {
            path
        }
    }

    pub fn is_copy(&self, node_id: &NodeId) -> bool {
        let relative_comfy_path = self
            .id_to_path
            .get(node_id)
            .unwrap()
            .clone()
            .remove(0)
            .unwrap();

        self.relative_path_to_is_copy
            .get(&relative_comfy_path)
            .cloned()
            .unwrap_or(false)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Ast> {
        self.localizations.iter()
    }

    pub fn reference_tree(&self) -> &Ast {
        &self.reference_tree
    }

    pub fn context_key(&self) -> Path {
        Path::root()
            .add_mod("crate".into())
            .set_ty("I18n".to_string().into())
    }

    pub fn get(&self, node_id: &NodeId) -> &Ast {
        let path = self.id_to_path.get(node_id).unwrap();
        let variant = path.root();
        self.get_variant(node_id, variant).unwrap()
    }

    pub fn get_variant_comfy_path(
        &self,
        node_id: &NodeId,
        variant: Identifier,
    ) -> comfy_i18n_ast::Path {
        self.id_to_path
            .get(node_id)
            .unwrap()
            .clone()
            .remove(0)
            .unwrap()
            .prepend(variant)
    }

    pub fn get_variant(&self, node_id: &NodeId, variant: Identifier) -> Option<&Ast> {
        self.localizations
            .iter()
            .find(|local| local.identifier == variant)
            .map(|ast| {
                ast.by_path(
                    &self
                        .get_variant_comfy_path(node_id, variant)
                        .remove(0)
                        .unwrap(),
                )
            })?
    }

    pub fn context_variant(&self, node_id: &NodeId) -> String {
        self.id_to_path.get(node_id).unwrap().root().to_string()
    }

    pub fn context_variant_identifier(&self, node_id: &NodeId) -> Identifier {
        self.id_to_path.get(node_id).unwrap().root()
    }

    pub fn context_variants(&self) -> impl Iterator<Item = Identifier> {
        self.localizations
            .iter()
            .map(|localization| localization.identifier.clone())
    }

    pub fn available_context_variants(&self, node_id: &NodeId) -> impl Iterator<Item = String> {
        self.context_variants()
            .filter(|id| {
                self.path_to_id
                    .contains_key(&self.get_variant_comfy_path(node_id, id.clone()))
            })
            .map(|id| id.to_string())
    }

    pub fn root_name(&self) -> NameSnakeCase {
        self.root_name.clone()
    }
}

fn create_is_copy_tree(
    node: &Ast,
    id_to_path: &HashMap<NodeId, comfy_i18n_ast::Path>,
    localizations: &Vec<Ast>,
    relative_path_to_is_copy: &mut HashMap<comfy_i18n_ast::Path, bool>,
) -> bool {
    let relative_comfy_path = id_to_path.get(&node.id).unwrap().clone().remove(0).unwrap();
    let is_copy = match &node.value {
        NodeValue::Literal(LiteralValue::String(StringValue::Template(_))) => false,
        NodeValue::Composite {
            children,
            value: CompositeValue::List { amount },
        } => {
            let same_size = localizations.iter().all(|localization| {
                if let Some(ast) = localization.by_path(&relative_comfy_path) {
                    if let NodeValue::Composite {
                        value:
                            CompositeValue::List {
                                amount: inner_amount,
                            },
                        ..
                    } = &ast.value
                    {
                        inner_amount == amount && !children.is_empty()
                    } else {
                        true
                    }
                } else {
                    true
                }
            });

            children.iter().all(|(_, it)| {
                create_is_copy_tree(it, id_to_path, localizations, relative_path_to_is_copy)
            }) && same_size
        }
        NodeValue::Composite { children, .. } => children.iter().all(|(_, it)| {
            create_is_copy_tree(it, id_to_path, localizations, relative_path_to_is_copy)
        }),
        _ => true,
    };
    relative_path_to_is_copy.insert(relative_comfy_path, is_copy);

    is_copy
}
