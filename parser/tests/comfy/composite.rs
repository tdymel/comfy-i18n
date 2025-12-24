use core::panic;

use comfy_i18n_ast::{CompositeValue, Identifier, LiteralValue, NodeValue, StringValue};
use comfy_i18n_parser::Parser;

#[test]
fn struct_composite() {
    let result = "{
        abc: \"test123\" 
}"
    .parse_struct()
    .expect("Parsing failed");
    if let NodeValue::Composite {
        children,
        value: CompositeValue::Struct,
    } = result
    {
        assert_eq!(children.len(), 1);
        let nodes = children.values().collect::<Vec<_>>();
        let child = nodes.first().unwrap();
        assert_eq!(child.identifier, Identifier::Field("abc".to_string()));
        if let NodeValue::Literal(LiteralValue::String(StringValue::Literal(str))) = &child.value {
            assert_eq!(str.as_str(), "test123");
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

#[test]
fn tuple() {
    let result = "(1, '2', \"3\")".parse_tuple().expect("Parsing failed");
    match result {
        NodeValue::Composite {
            children,
            value: CompositeValue::Tuple,
        } => {
            assert_eq!(children.len(), 3);
        }
        _ => panic!(),
    }
}

#[test]
fn list() {
    let result = "[1,2,3]".parse_list().expect("Parsing failed");
    match result {
        NodeValue::Composite {
            children,
            value: CompositeValue::List { amount: 3 },
        } => {
            assert_eq!(children.len(), 3);
        }
        _ => panic!(),
    }
}

#[test]
fn list_repeat() {
    let result = "[1; 3]".parse_list().expect("Parsing failed");
    match result {
        NodeValue::Composite {
            children,
            value: CompositeValue::List { amount: 3 },
        } => {
            assert_eq!(children.len(), 1);
        }
        _ => panic!(),
    }
}

#[test]
fn composite() {
    let result = "[([1,2,3], [1; 4], (1,'2', b'3'), {
        arr_repeat: [1;2],
        tuple: (1, '2', \"3\"),
        nested_struct: {
            wambo: [1,2,3]
        }
    }) ;2]"
        .parse_node_value()
        .expect("Parsing failed!");
    match result {
        NodeValue::Composite {
            children,
            value: CompositeValue::List { amount: 2 },
        } => {
            assert_eq!(children.len(), 1);
            let nodes = children.values().collect::<Vec<_>>();
            let child = nodes.first().unwrap();
            match &child.value {
                NodeValue::Composite {
                    children,
                    value: CompositeValue::Tuple,
                } => {
                    assert_eq!(children.len(), 4);
                    assert!(children.iter().any(|child| matches!(
                        child.1.value,
                        NodeValue::Composite {
                            value: CompositeValue::Tuple,
                            ..
                        }
                    )));
                    assert!(children.iter().any(|child| matches!(
                        child.1.value,
                        NodeValue::Composite {
                            value: CompositeValue::Struct,
                            ..
                        }
                    )));
                    assert!(children.iter().any(|child| matches!(
                        child.1.value,
                        NodeValue::Composite {
                            value: CompositeValue::List { amount: 3 },
                            ..
                        }
                    )));
                    assert!(children.iter().any(|child| matches!(
                        child.1.value,
                        NodeValue::Composite {
                            value: CompositeValue::List { amount: 4 },
                            ..
                        }
                    )));
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}
