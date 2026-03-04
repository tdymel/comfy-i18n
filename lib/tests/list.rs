use comfy_i18n_macro::{i18n, i18n_init};

i18n_init!(
    #[fallback]
    DE,
    EN,
);

i18n!(
    Happy,
    DE: {
        list1: [1; 10],
        list2: [1,2,3,4],
    },
    EN: {
        list1: [2; 10],
        list2: [5,6,7,8],
    }
);

#[test]
fn happy() {
    assert_eq!(I18n::DE.happy().list1().value(), &[1; 10]);
    assert_eq!(I18n::DE.happy().list2().value(), &[1, 2, 3, 4]);

    assert_eq!(I18n::EN.happy().list1().value(), &[2; 10]);
    assert_eq!(I18n::EN.happy().list2().value(), &[5, 6, 7, 8]);
}

i18n!(
    FallbackTest,
    DE: {
        list1: [1; 10],
    },
);

#[test]
fn fallback() {
    assert_eq!(
        I18n::DE.fallback_test().list1().value(),
        I18n::EN.fallback_test().list1().value()
    );
}

i18n!(
    Nested,
    DE: {
        value: [[[2; 2], [3; 2]]; 1],
    },
);

#[test]
fn nested() {
    assert_eq!(I18n::DE.nested().value()[0][0].value(), &[2; 2]);
    assert_eq!(I18n::DE.nested().value()[0][1].value(), &[3; 2]);
}
