use std::collections::HashMap;

use comfy_i18n_ast::{Ast, CompositeValue, Identifier, LiteralValue, NodeValue};

#[test]
fn merge_add_child() {
    let mut child_map1 = HashMap::new();
    let child1 = Ast::new(
        Identifier::Field("b".to_string()),
        NodeValue::Literal(LiteralValue::Char('C')),
    );
    child_map1.insert(child1.identifier.clone(), child1);

    let mut ast1 = Ast::new(
        Identifier::Field("a".to_string()),
        NodeValue::Composite {
            children: child_map1,
            value: CompositeValue::Struct,
        },
    );

    let mut child_map2 = HashMap::new();
    let child2 = Ast::new(
        Identifier::Field("c".to_string()),
        NodeValue::Literal(LiteralValue::Char('C')),
    );
    child_map2.insert(child2.identifier.clone(), child2);

    let ast2 = Ast::new(
        Identifier::Field("a".to_string()),
        NodeValue::Composite {
            children: child_map2,
            value: CompositeValue::Struct,
        },
    );

    ast1.merge(ast2);

    assert!(ast1.get(&Identifier::Field("b".to_string())).is_some());
    assert!(ast1.get(&Identifier::Field("c".to_string())).is_some());
}

#[test]
fn merge_nested_add_child() {
    let mut child_map1 = HashMap::new();
    let child1 = Ast::new(
        Identifier::Field("b".to_string()),
        NodeValue::Composite {
            children: HashMap::new(),
            value: CompositeValue::Struct,
        },
    );
    child_map1.insert(child1.identifier.clone(), child1);

    let mut ast1 = Ast::new(
        Identifier::Field("a".to_string()),
        NodeValue::Composite {
            children: child_map1,
            value: CompositeValue::Struct,
        },
    );

    let mut child_map2 = HashMap::new();
    let nested_child = Ast::new(
        Identifier::Field("c".to_string()),
        NodeValue::Literal(LiteralValue::Char('C')),
    );
    let mut nested_child_map = HashMap::new();
    nested_child_map.insert(nested_child.identifier.clone(), nested_child);
    let child2 = Ast::new(
        Identifier::Field("b".to_string()),
        NodeValue::Composite {
            children: nested_child_map,
            value: CompositeValue::Struct,
        },
    );
    child_map2.insert(child2.identifier.clone(), child2);

    let ast2 = Ast::new(
        Identifier::Field("a".to_string()),
        NodeValue::Composite {
            children: child_map2,
            value: CompositeValue::Struct,
        },
    );

    ast1.merge(ast2);

    let child = ast1.get(&Identifier::Field("b".to_string())).unwrap();

    assert!(child.get(&Identifier::Field("c".to_string())).is_some());
}

#[test]
fn merge_conflict_keep_b() {
    let mut child_map1 = HashMap::new();
    let child1 = Ast::new(
        Identifier::Field("b".to_string()),
        NodeValue::Literal(LiteralValue::Char('C')),
    );
    child_map1.insert(child1.identifier.clone(), child1);

    let mut ast1 = Ast::new(
        Identifier::Field("a".to_string()),
        NodeValue::Composite {
            children: child_map1,
            value: CompositeValue::Struct,
        },
    );

    let mut child_map2 = HashMap::new();
    let child2 = Ast::new(
        Identifier::Field("b".to_string()),
        NodeValue::Literal(LiteralValue::Char('D')),
    );
    child_map2.insert(child2.identifier.clone(), child2);

    let ast2 = Ast::new(
        Identifier::Field("a".to_string()),
        NodeValue::Composite {
            children: child_map2,
            value: CompositeValue::Struct,
        },
    );

    ast1.merge(ast2);

    let child = ast1.get(&Identifier::Field("b".to_string())).unwrap();
    if let NodeValue::Literal(LiteralValue::Char(value)) = child.value {
        assert_eq!(value, 'C');
    } else {
        panic!();
    }
}
