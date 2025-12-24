use comfy_i18n_parser::Parser;

#[test]
fn empty_list() {
    let result = "[;2]".parse_node_value();
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .starts_with("unsupported expression")
    );
}

#[test]
fn empty_list2() {
    let result = "[]".parse_node_value();
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .starts_with("unexpected end of input")
    );
}
