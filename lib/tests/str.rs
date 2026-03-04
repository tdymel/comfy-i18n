use comfy_i18n_macro::{i18n, i18n_init};

i18n_init!(
    #[fallback]
    DE,
    EN,
);

i18n!(
    happy,
    DE: {
        str_value: "DE VALUE",
    },
    EN: {
        str_value: "EN VALUE",
    }
);

#[test]
fn happy() {
    assert_eq!(I18n::DE.happy().str_value(), &"DE VALUE");
    assert_eq!(I18n::EN.happy().str_value(), &"EN VALUE");
}

i18n!(
    fallback_test,
    DE: {
        str_value: "DE VALUE",
    },
);

#[test]
fn fallback() {
    assert_eq!(
        I18n::DE.fallback_test().str_value(),
        I18n::EN.fallback_test().str_value()
    );
}

i18n!(
    cast,
    DE: {
        casted_value: crate::I18n::DE.happy().str_value() as &'static str,
    },
);

#[test]
fn cast() {
    assert_eq!(I18n::DE.cast().casted_value(), I18n::DE.happy().str_value());
}

#[test]
fn t_macro() {
    assert!(true);
}
