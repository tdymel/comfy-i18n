use comfy_i18n_ast::Ast;
use comfy_i18n_generator::{NameSnakeCase, RustGenerator};
use comfy_i18n_parser::Parser;

#[test]
fn poc() {
    let ast: Ast = "sth: {
        de: {
            some_value: \"Test\",
            int: 42,
            flt: 3.14,
            list1: [1i128; 42],
            list2: [1,2,3,4,5],
            list3: [(1,'2'); 5],
            list4: [(1,2), (3,4), (5,6)],
            list5: [{ test: (1,2) }; 5],
            unnamed_nested: [[[[[[{ test: 1 }]; 1]; 1]; 1]; 1]; 1],
            nested_struct: {
                f1: 'W'
            }   
        }
    }"
    .parse_field()
    .unwrap()
    .into();

    let token_stream = ast.to_rust();
    // let token_stream = RustGeneratorV2::new(ast, "root".into()).generate();

    println!("{}", token_stream);
    assert!(false);
}
