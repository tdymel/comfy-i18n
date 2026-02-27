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
            f32_value: 3.14f32,
            f64_value: 3.14f32,
        },
        EN: {
            f32_value: 3.15f32,
            f64_value: 3.15f32,
        }
    }
);

#[test]
fn happy() {
    assert_eq!(Language::DE.happy().f32_value(), &3.14f32);
    assert_eq!(Language::DE.happy().f64_value(), &3.14f32);

    assert_eq!(Language::EN.happy().f32_value(), &3.15f32);
    assert_eq!(Language::EN.happy().f64_value(), &3.15f32);
}

i18n!(
    name: FallbackTest,
    key: crate::Language,
    translations: {
        DE: {
            f32_value: 3.14f32,
        },
    }
);

#[test]
fn fallback() {
    assert_eq!(
        Language::DE.fallback_test().f32_value(),
        Language::EN.fallback_test().f32_value()
    );
}
