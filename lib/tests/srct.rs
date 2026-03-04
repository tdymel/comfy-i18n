use comfy_i18n_macro::{i18n, i18n_init};

i18n_init!(
    #[fallback]
    DE,
    EN,
);

i18n!(
    happy,
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
);

#[test]
fn happy() {
    assert_eq!(I18n::DE.happy().value().sth(), &true);

    assert_eq!(I18n::EN.happy().value().sth(), &false);
}

i18n!(
    fallback_test,
    DE: {
        value: {
            sth: true
        }
    },
);

#[test]
fn fallback() {
    assert_eq!(
        I18n::DE.fallback_test().value().sth(),
        I18n::EN.fallback_test().value().sth()
    );
}

i18n!(
    nested,
    DE: {
        value: {
            sth: {
                other: true
            }
        }
    },
);

#[test]
fn nested() {
    assert_eq!(I18n::DE.nested().value().sth().other(), &true);
}
