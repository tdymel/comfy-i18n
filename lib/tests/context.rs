use comfy_i18n_macro::ComfyI18n;

#[derive(ComfyI18n, Debug)]
pub enum Language {
    #[fallback]
    DE,
    EN,
}

#[test]
fn context_values() {
    assert_eq!(Language::DE.context().fallback, true);
    assert_eq!(Language::EN.context().fallback, false);
}

#[test]
fn amount() {
    assert_eq!(Language::amount(), 2);
}

#[test]
fn fallback() {
    assert_eq!(
        Language::DE.fallback([Some(Language::EN), None]),
        Language::EN
    )
}
