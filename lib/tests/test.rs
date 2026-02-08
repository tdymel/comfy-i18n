use comfy_i18n_macro::i18n;

pub enum Language {}

#[test]
fn poc() {
    i18n!(
        name: Test,
        key: Language,
        translations: {
            de: {
                some_value: "Test",
                dfmt: "Hallo, {world} und andere args: {}!",
                int: 42,
                flt: 3.14,
                list1: [1i128; 42],
                list2: [1,2,3,4,5],
                list3: [(1,'2'); 5],
                list4: [(1,2), (3,4), (5,6)],
                list5: [{ test: (1,2) }; 5],
                unnamed_nested: [[[[[[{ test: 1 }]; 1]; 1]; 1]; 1]; 1],
                tuple_struct: ({ a: "Wambo" }, {b: 42 }, 'c'),
                nested_struct: {
                    f1: 'W'
                }
            },
            en: {
                some_value: "Test",
                dfmt: "Hello, {world} and some other arg: {}!",
                int: 42,
                flt: 3.14,
                list1: [1i128; 42],
                list2: [1,2,3,4,5],
                list3: [(1,'2'); 5],
                list4: [(1,2), (3,4), (5,6)],
                list5: [{ test: (1,2) }; 5],
                unnamed_nested: [[[[[[{ test: 1 }]; 1]; 1]; 1]; 1]; 1],
                tuple_struct: ({ a: "Wambo" }, {b: 42 }, 'c'),
                nested_struct: {
                    f1: 'W'
                }
            }
        }
    );

    assert_eq!(
        test::TEST_DE.dfmt(&"World", &42),
        test::TEST_EN.dfmt(&"World", &42)
    );
    assert!(false);
}
