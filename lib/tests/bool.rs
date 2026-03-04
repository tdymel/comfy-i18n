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
            value: true,
        },
        EN: {
            value: false
        }
    }
);

#[test]
fn happy() {
    assert_eq!(I18n::DE.happy().value(), &true);

    assert_eq!(I18n::EN.happy().value(), &false);
}

i18n!(
    name: FallbackTest,
    key: crate::I18n,
    translations: {
        DE: {
            value: true,
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
