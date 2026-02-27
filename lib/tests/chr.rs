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
            value: 'A',
        },
        EN: {
            value: 'B'
        }
    }
);

#[test]
fn happy() {
    assert_eq!(Language::DE.happy().value(), &'A');

    assert_eq!(Language::EN.happy().value(), &'B');
}

i18n!(
    name: FallbackTest,
    key: crate::Language,
    translations: {
        DE: {
            value: 'A',
        },
    }
);

#[test]
fn fallback() {
    assert_eq!(
        Language::DE.fallback_test().value(),
        Language::EN.fallback_test().value()
    );
}
