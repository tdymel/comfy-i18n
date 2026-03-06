use std::collections::HashMap;

use comfy_i18n_ast::{Ast, Identifier, NodeId};

use crate::{rust_generator::Path, shared::NameSnakeCase};

pub struct Context {
    root_name: NameSnakeCase,
    localizations: Vec<Ast>,
    id_to_path: HashMap<NodeId, comfy_i18n_ast::Path>,
    path_to_id: HashMap<comfy_i18n_ast::Path, NodeId>,
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

        Self {
            root_name,
            localizations,
            id_to_path,
            path_to_id,
            reference_tree
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
