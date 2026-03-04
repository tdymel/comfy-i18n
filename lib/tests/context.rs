use comfy_i18n_macro::i18n_init;

i18n_init!(
    #[fallback]
    DE,
    EN,
);

#[test]
fn context_values() {
    assert_eq!(I18n::DE.context().fallback, true);
    assert_eq!(I18n::EN.context().fallback, false);
}

#[test]
fn amount() {
    assert_eq!(I18n::amount(), 2);
}

#[test]
fn fallback() {
    assert_eq!(I18n::DE.fallback([Some(I18n::EN), None]), I18n::EN)
}
