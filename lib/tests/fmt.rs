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
            value: "Hallo, {world}!",
        },
        EN: {
            value: "Hello, {world}!"
        }
    }
);

#[test]
fn happy() {
    assert_eq!(Language::DE.happy().value(&"Welt"), "Hallo, Welt!");

    assert_eq!(Language::EN.happy().value(&"World"), "Hello, World!");
}

i18n!(
    name: FallbackTest,
    key: crate::Language,
    translations: {
        DE: {
            value: "Hallo, {world}!",
        },
    }
);

#[test]
fn fallback() {
    assert_eq!(Language::DE.fallback_test().value(&"Welt"), "Hallo, Welt!");

    assert_eq!(Language::EN.fallback_test().value(&"World"), "Hallo, World!");
}
