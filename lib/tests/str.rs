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
            str_value: "DE VALUE",
        },
        EN: {
            str_value: "EN VALUE",
        }
    }
);

#[test]
fn happy() {
    assert_eq!(Language::DE.happy().str_value(), &"DE VALUE");
    assert_eq!(Language::EN.happy().str_value(), &"EN VALUE");
}

i18n!(
    name: FallbackTest,
    key: crate::Language,
    translations: {
        DE: {
            str_value: "DE VALUE",
        },
    }
);

#[test]
fn fallback() {
    assert_eq!(
        Language::DE.fallback_test().str_value(),
        Language::EN.fallback_test().str_value()
    );
}

i18n!(
    name: Cast,
    key: crate::Language,
    translations: {
        DE: {
            casted_value: crate::Language::DE.happy().str_value() as &'static str,
        },
    }
);

#[test]
fn cast() {
    assert_eq!(
        Language::DE.cast().casted_value(),
        Language::DE.happy().str_value()
    );
}

#[test]
fn t_macro() {
    assert!(true);
}
