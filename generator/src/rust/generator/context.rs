use std::collections::HashMap;

use comfy_i18n_ast::{Ast, NodeId};

use crate::{generator::Path, shared::NameSnakeCase};

pub struct Context {
    root_name: NameSnakeCase,
    context_key: Path,
    localizations: Vec<Ast>,
    id_to_path: HashMap<NodeId, comfy_i18n_ast::Path>,
}

impl Context {
    pub fn new(localization_tree: Ast, root_name: NameSnakeCase, context_key: Path) -> Self {
        let localizations = localization_tree
            .children_into()
            .expect("Root node must contain children")
            .map(|mut it| {
                it.detach_from_parent();
                it
            })
            .collect::<Vec<_>>();

        let id_to_path = localizations
            .iter()
            .fold(HashMap::new(), |mut acc, context| {
                let map = context.id_to_path_map();
                acc.extend(map);
                acc
            });

        Self {
            root_name,
            context_key,
            localizations,
            id_to_path,
        }
    }

    pub fn relative_path(&self, node_id: &NodeId) -> Path {
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

    pub fn main(&self) -> &Ast {
        self.localizations.first().unwrap()
    }

    pub fn context_key(&self) -> &Path {
        &self.context_key
    }

    pub fn context_variant(&self, node_id: &NodeId) -> String {
        self.id_to_path.get(node_id).unwrap().root().to_string()
    }
}
