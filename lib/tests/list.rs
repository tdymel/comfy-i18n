use comfy_i18n_macro::{ComfyI18n, i18n};

#[derive(ComfyI18n)]
pub enum Language {
    #[fallback]
    DE,
    EN,
}

i18n!(
    name: Happy,
    key: crate::Language,
    translations: {
        DE: {
            list1: [1; 10],
            list2: [1,2,3,4],
        },
        EN: {
            list1: [2; 10],
            list2: [5,6,7,8],
        }
    }
);

#[test]
fn happy() {
    assert_eq!(Language::DE.happy().list1(), [1; 10]);
    assert_eq!(Language::DE.happy().list2(), [1, 2, 3, 4]);

    assert_eq!(Language::EN.happy().list1(), [2; 10]);
    assert_eq!(Language::EN.happy().list2(), [5, 6, 7, 8]);
}

i18n!(
    name: FallbackTest,
    key: crate::Language,
    translations: {
        DE: {
            list1: [1; 10],
        },
    }
);

#[test]
fn fallback() {
    assert_eq!(
        Language::DE.fallback_test().list1(),
        Language::EN.fallback_test().list1()
    );
}

i18n!(
    name: Nested,
    key: crate::Language,
    translations: {
        DE: {
            value: [[[2; 2], [3; 2]]; 1],
        },
    }
);

#[test]
fn nested() {
    assert_eq!(Language::DE.nested().value(), [[[2; 2], [3; 2]]; 1]);
}
