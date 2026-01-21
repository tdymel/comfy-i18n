use std::collections::HashMap;

use comfy_i18n_ast::{
    Ast, CompositeValue, FloatValue, Identifier, IntegerValue, LiteralValue, NodeId, NodeValue,
    Path, StringValue,
};
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::rust::{
    initialization::{FieldValue, Initialization, RustValue},
    module::Module,
    name::NameSnakeCase,
    strct::{Field, RustType, Struct},
};

use super::initialization::VariableType;

struct AstContext {
    contexts: Vec<Ast>,
    id_to_path: HashMap<NodeId, Path>,
}

impl AstContext {
    pub fn map(ast_with_contexts: Ast) -> Module {
        let contexts = ast_with_contexts
            .children_into()
            .expect("Root node must contain children")
            .map(|mut it| {
                it.detach_from_parent();
                it
            })
            .collect::<Vec<_>>();

        let id_to_path = contexts.iter().fold(HashMap::new(), |mut acc, context| {
            let map = context.id_to_path_map();
            acc.extend(map);
            acc
        });

        let ast_context = Self {
            contexts,
            id_to_path,
        };

        let node = ast_context.contexts.get(0).unwrap();
        let mut module = ast_context.to_module(node);
        let relative_path = ast_context.relative_path(&node.id);
        module.initializations = ast_context
            .contexts
            .iter()
            .map(|context| ast_context.to_initialization(context.by_path(&relative_path).unwrap()))
            .collect::<Vec<_>>();

        module
    }

    fn relative_path(&self, id: &NodeId) -> Path {
        self.id_to_path.get(id).unwrap().clone().remove(0).unwrap()
    }

    fn context_name(&self, id: &NodeId) -> Identifier {
        self.id_to_path.get(id).unwrap().root()
    }

    fn to_module(&self, ast: &Ast) -> Module {
        let name = NameSnakeCase::from(ast.identifier.clone());
        let strct = self.to_struct(ast);
        let modules = match &ast.value {
            NodeValue::Composite { children, .. } => children
                .iter()
                .filter(|(_, v)| matches!(v.value, NodeValue::Composite { .. }))
                .map(|(_, v)| self.to_module(v))
                .collect(),
            NodeValue::Literal(_) => vec![],
        };

        Module {
            name,
            strct: Some(strct),
            initializations: Vec::default(),
            modules,
        }
    }

    fn to_struct(&self, ast: &Ast) -> Struct {
        let strct = Struct::new(NameSnakeCase::from(ast.identifier.clone()).to_pascal_case());
        match &ast.value {
            NodeValue::Composite {
                children,
                value: CompositeValue::Struct,
            } => {
                let fields = children
                    .iter()
                    .map(|(_, v)| self.to_field(v))
                    .collect::<Vec<_>>();

                strct.with_fields(fields)
            }
            _ => strct,
        }
    }

    fn to_field(&self, ast: &Ast) -> Field {
        let name = ast.identifier.clone().into();
        let ty = self.to_rust_type(ast);
        // let ty = to_rust_type(ast, current_context, contexts, Vec::new(), false);

        Field { name, ty }
    }

    fn to_initialization(&self, ast: &Ast) -> Initialization {
        let name =
            NameSnakeCase::from(ast.identifier.clone()).concat(self.context_name(&ast.id).into());
        let ty = self.to_rust_type(ast);
        let value = self.to_rust_value(ast);

        // TODO: Try to find out if we can make a variable const
        Initialization {
            var_ty: VariableType::Const,
            ty,
            name,
            value,
        }
    }

    fn to_rust_type(&self, ast: &Ast) -> RustType {
        match &ast.value {
            NodeValue::Composite { children, value } => match value {
                CompositeValue::Struct => RustType::Struct {
                    mod_names: self.relative_path(&ast.id).map(|id| id.clone().into()),
                    name: NameSnakeCase::from(ast.identifier.clone()).to_pascal_case(),
                },
                CompositeValue::Tuple => {
                    let mut pairs = children.iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
                    pairs.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
                    let types = pairs
                        .into_iter()
                        .map(|(_, v)| self.to_rust_type(v))
                        .collect::<Vec<_>>();

                    RustType::Tuple(types)
                }
                CompositeValue::List { amount: list_size } => {
                    let path = self.relative_path(&ast.id);
                    let amount_the_same = self.contexts.iter().all(|context| {
                        matches!(
                            context.by_path(&path).map(|it| &it.value),
                            Some(&NodeValue::Composite {
                                value: CompositeValue::List { amount },
                                ..
                            }) if amount == *list_size
                        )
                    });

                    let ty = self.to_rust_type(children.get(&Identifier::Element(0)).unwrap());
                    let amount = if amount_the_same {
                        Some(*list_size)
                    } else {
                        None
                    };
                    RustType::List {
                        ty: Box::new(ty),
                        amount,
                    }
                }
            },
            NodeValue::Literal(literal_value) => match literal_value {
                LiteralValue::String(string_value) => match string_value {
                    StringValue::Literal(_) => RustType::String,
                    StringValue::Template(_template) => todo!(),
                },
                LiteralValue::Char(_) => RustType::Char,
                LiteralValue::Float(float_value) => match float_value {
                    FloatValue::F64(_) => RustType::Float { bits: 64 },
                    FloatValue::F32(_) => RustType::Float { bits: 32 },
                },
                LiteralValue::Integer(integer_value) => match integer_value {
                    IntegerValue::I128(_) => RustType::Integer {
                        unsigned: false,
                        bits: 128,
                    },
                    IntegerValue::U128(_) => RustType::Integer {
                        unsigned: true,
                        bits: 128,
                    },
                    IntegerValue::I64(_) => RustType::Integer {
                        unsigned: false,
                        bits: 64,
                    },
                    IntegerValue::U64(_) => RustType::Integer {
                        unsigned: true,
                        bits: 64,
                    },
                    IntegerValue::I32(_) => RustType::Integer {
                        unsigned: false,
                        bits: 32,
                    },
                    IntegerValue::U32(_) => RustType::Integer {
                        unsigned: true,
                        bits: 32,
                    },
                    IntegerValue::I16(_) => RustType::Integer {
                        unsigned: false,
                        bits: 16,
                    },
                    IntegerValue::U16(_) => RustType::Integer {
                        unsigned: true,
                        bits: 16,
                    },
                    IntegerValue::I8(_) => RustType::Integer {
                        unsigned: false,
                        bits: 8,
                    },
                    IntegerValue::U8(_) => RustType::Integer {
                        unsigned: true,
                        bits: 8,
                    },
                },
                LiteralValue::Bool(_) => RustType::Bool,
                LiteralValue::Cast { .. } => todo!(),
            },
        }
    }

    fn to_rust_value(&self, ast: &Ast) -> RustValue {
        match &ast.value {
            NodeValue::Composite { children, value } => {
                let mut pairs = children.iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
                pairs.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
                let mut values = pairs
                    .into_iter()
                    .map(|(k, v)| (k, self.to_rust_value(v)))
                    .collect::<Vec<_>>();

                match value {
                    CompositeValue::Struct => RustValue::Struct {
                        mod_names: self.relative_path(&ast.id).map(|id| id.clone().into()),
                        name: NameSnakeCase::from(ast.identifier.clone()).to_pascal_case(),
                        fields: values
                            .into_iter()
                            .map(|(k, v)| FieldValue {
                                name: k.clone().into(),
                                value: v,
                            })
                            .collect(),
                    },
                    CompositeValue::Tuple => {
                        RustValue::Tuple(values.into_iter().map(|(_, v)| v).collect())
                    }
                    CompositeValue::List { amount: list_size } => {
                        let path = self.relative_path(&ast.id);
                        let amount_the_same = self.contexts.iter().all(|context| {
                            matches!(
                                context.by_path(&path).map(|it| &it.value),
                                Some(&NodeValue::Composite {
                                    value: CompositeValue::List { amount },
                                    ..
                                }) if amount == *list_size
                            )
                        });

                        if amount_the_same {
                            let (_, value) = values.remove(0);
                            RustValue::ListRepeated {
                                value: Box::new(value),
                                amount: *list_size,
                            }
                        } else {
                            RustValue::List(values.into_iter().map(|(_, v)| v).collect())
                        }
                    }
                }
            }
            NodeValue::Literal(literal_value) => match literal_value {
                LiteralValue::String(string_value) => RustValue::String(string_value.clone()),
                LiteralValue::Char(val) => RustValue::Char(*val),
                LiteralValue::Float(float_value) => RustValue::Float(float_value.clone()),
                LiteralValue::Integer(integer_value) => RustValue::Integer(integer_value.clone()),
                LiteralValue::Bool(val) => RustValue::Bool(*val),
                LiteralValue::Cast { expression } => todo!(),
            },
        }
    }
}

// TODO: Make output deterministic
pub trait RustGenerator {
    fn to_rust(self) -> TokenStream;
}

impl RustGenerator for Ast {
    fn to_rust(self) -> TokenStream {
        AstContext::map(self).to_token_stream()
    }
}
