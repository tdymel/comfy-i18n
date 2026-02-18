#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

use comfy_i18n_macro::{ComfyI18n, i18n};

/*
# Fallback via functions?
* Make properties private, access via functions DE.test().a().b()
* Tuples and Arrays have to get some sort of Wrapper object, that allows HackFn deref and also things like get_by_path impls later
* Properites would have to be Optional, as we are now merging the trees and some values may not exist
* TODO: For remote translations we would have to prefetch them before, to at least know about existance of attributes.

// For Arrays and Tuples, we gotta use the HackFn way to create an indirection
// This should be packed into a macro later, so people can implement their own functions
impl test::nested_struct::NestedStruct {
    pub const fn f1(&self) -> char {
        let self_lang = Language::DE;
        match self_lang.fallback([Some(Language::DE), None]) {
            Language::DE => self.f1,
            // Language::EN => EN.test().a().b().f1
            _ => unreachable!()
        }
    }
}

# t! - Macro
* For static paths go static route
* For dynamic paths go get_by_path route

# Optimization
* Create specialized fallback function per struct, so we dont have to repeat the available lang code all the time

*/

#[derive(ComfyI18n)]
pub enum Language {
    #[fallback]
    DE,
    EN,
}

i18n!(
    name: Test,
    key: crate::Language,
    translations: {
        DE: {
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
        EN: {
            some_value: "Test_en",
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

#[test]
fn poc() {
    assert_eq!(
        Language::DE.test().some_value(),
        Language::EN.test().some_value()
    );
    assert_eq!(
        Language::DE.test().dfmt(&"World", &42),
        Language::EN.test().dfmt(&"World", &42)
    );
    assert!(false);
}
