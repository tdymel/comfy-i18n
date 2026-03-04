use comfy_i18n::{i18n, i18n_init};

i18n_init!(DE, EN);

i18n!(
    happy,
    DE: "tests/locales/de.comfy"
);

#[test]
fn happy() {
    assert_eq!(I18n::DE.happy().hello(), &"world");
}
