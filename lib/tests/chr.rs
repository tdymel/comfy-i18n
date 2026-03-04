use comfy_i18n_macro::{i18n, i18n_init};

i18n_init!(
    #[fallback]
    DE,
    EN,
);

i18n!(
    name: Happy,
    key: crate::I18n,
    translations: {
        DE: {
            value: 'A',
        },
        EN: {
            value: 'B'
        }
    }
);

#[test]
fn happy() {
    assert_eq!(I18n::DE.happy().value(), &'A');

    assert_eq!(I18n::EN.happy().value(), &'B');
}

i18n!(
    name: FallbackTest,
    key: crate::I18n,
    translations: {
        DE: {
            value: 'A',
        },
    }
);

#[test]
fn fallback() {
    assert_eq!(
        I18n::DE.fallback_test().value(),
        I18n::EN.fallback_test().value()
    );
}
