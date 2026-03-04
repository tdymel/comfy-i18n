use comfy_i18n_macro::{i18n, i18n_init};

i18n_init!(
    #[fallback]
    DE,
    EN,
);

i18n!(
    happy,
    DE: {
        f32_value: 3.14f32,
        f64_value: 3.14f32,
    },
    EN: {
        f32_value: 3.15f32,
        f64_value: 3.15f32,
    }
);

#[test]
fn happy() {
    assert_eq!(I18n::DE.happy().f32_value(), &3.14f32);
    assert_eq!(I18n::DE.happy().f64_value(), &3.14f32);

    assert_eq!(I18n::EN.happy().f32_value(), &3.15f32);
    assert_eq!(I18n::EN.happy().f64_value(), &3.15f32);
}

i18n!(
    fallback_test,
    DE: {
        f32_value: 3.14f32,
    },
);

#[test]
fn fallback() {
    assert_eq!(
        I18n::DE.fallback_test().f32_value(),
        I18n::EN.fallback_test().f32_value()
    );
}
