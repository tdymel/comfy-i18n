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
            value: {
                sth: true
            },
        },
        EN: {
            value: {
                sth: false
            }
        }
    }
);

#[test]
fn happy() {
    assert_eq!(Language::DE.happy().value().sth(), true);

    assert_eq!(Language::EN.happy().value().sth(), false);
}

i18n!(
    name: FallbackTest,
    key: crate::Language,
    translations: {
        DE: {
            value: {
                sth: true
            }
        },
    }
);

#[test]
fn fallback() {
    assert_eq!(
        Language::DE.fallback_test().value().sth(),
        Language::EN.fallback_test().value().sth()
    );
}

i18n!(
    name: Nested,
    key: crate::Language,
    translations: {
        DE: {
            value: {
                sth: {
                    other: true
                }
            }
        },
    }
);

#[test]
fn nested() {
    assert_eq!(Language::DE.nested().value().sth().other(), true);
}
