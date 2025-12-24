use comfy_i18n_parser::Parser;
use criterion::{Criterion, criterion_group, criterion_main};

fn parsing_performance(c: &mut Criterion) {
    c.bench_function("average case", |b| {
        b.iter(|| {
            "
            DE: {
                field: \"Test\",
                arr_field: [1,2,3,4],
                str_field: \"Wambo\",
                bool_field: true,
                int_field: 322,
                float_field_renamed: 42.3,
                char_field: 'C',
                byte_field: 0x41,
                byte_str_field: b\"Hello World\",
                tuple_field: (1, \"2\", '3'),
                arr_struct_field: [
                    {
                        a: \"a\",
                        b: \"b\"
                    }
                ],
                arr_repeat_field: [1; 5],
                arr_struct_repeat_field: [{
                    hello: \"WORLD\"
                }; 10],
                arr_arr_field: [[1,2], [3,4]],
                arr_arr_struct_field: [[{
                    a: \"sth\"
                }]],
                tuple_struct_field: (1, \"2\", {
                    a: \"3\"
                }, [1,2,3,4]),
                nested: {
                    test: \"123\",
                    nested_two: {
                        some_field: \"Test123\"
                    }
                }
            },
            EN: {
                field: \"Test\",
                arr_field: [1,2,3,4],
                str_field: \"Wambo\",
                bool_field: true,
                int_field: 322,
                float_field_renamed: 42.3,
                char_field: 'C',
                byte_field: 0x41,
                byte_str_field: b\"Hello World\",
                tuple_field: (1, \"2\", '3'),
                arr_struct_field: [
                    {
                        a: \"a\",
                        b: \"b\"
                    }
                ],
                arr_repeat_field: [1; 5],
                arr_struct_repeat_field: [{
                    hello: \"WORLD\"
                }; 10],
                arr_arr_field: [[1,2], [3,4]],
                arr_arr_struct_field: [[{
                    a: \"sth\"
                }]],
                tuple_struct_field: (1, \"2\", {
                    a: \"3\"
                }, [1,2,3,4]),
                nested: {
                    test: \"123\",
                    nested_two: {
                        some_field: \"Test123\"
                    }
                }
            }
        "
            .parse_fields()
            .unwrap();
        })
    });
}

criterion_group!(benches, parsing_performance);
criterion_main!(benches);
