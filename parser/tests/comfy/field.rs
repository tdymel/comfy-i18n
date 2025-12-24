use comfy_i18n_ast::{Identifier, LiteralValue, NodeValue};
use comfy_i18n_parser::Parser;

#[test]
fn field() {
    let result = "some_field: 'A'".parse_field().expect("Parsing failed");

    assert_eq!(
        result.identifier,
        Identifier::Field("some_field".to_string())
    );
    assert!(matches!(
        result.value,
        NodeValue::Literal(LiteralValue::Char(..))
    ));
}
