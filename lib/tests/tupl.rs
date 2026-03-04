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
            value: (1, '2', "3"),
        },
        EN: {
            value: (4, '5', "6"),
        }
    }
);

#[test]
fn happy() {
    assert_eq!(Language::DE.happy().value().0(), &1);
    assert_eq!(Language::DE.happy().value().1(), &'2');
    assert_eq!(Language::DE.happy().value().2(), &"3");
    assert_eq!(Language::DE.happy().value().value(), (&1, &'2', &"3"));

    assert_eq!(Language::EN.happy().value().0(), &4);
    assert_eq!(Language::EN.happy().value().1(), &'5');
    assert_eq!(Language::EN.happy().value().2(), &"6");
    assert_eq!(Language::EN.happy().value().value(), (&4, &'5', &"6"));
}

i18n!(
    name: FallbackTest,
    key: crate::Language,
    translations: {
        DE: {
            value: (1, '2', "3"),
        },
    }
);

#[test]
fn fallback() {
    assert_eq!(
        Language::DE.fallback_test().value().value(),
        Language::EN.fallback_test().value().value()
    );
}

i18n!(
    name: Nested,
    key: crate::Language,
    translations: {
        DE: {
            value: (1, (2, (3, (4, (5, 6))))),
        },
    }
);

#[test]
fn nested() {
    assert_eq!(Language::DE.nested().value().0(), &1);
    assert_eq!(Language::DE.nested().value().1().0(), &2);
    assert_eq!(Language::DE.nested().value().1().1().0(), &3);
    assert_eq!(Language::DE.nested().value().1().1().1().0(), &4);
    assert_eq!(Language::DE.nested().value().1().1().1().1().0(), &5);
    assert_eq!(Language::DE.nested().value().1().1().1().1().1(), &6);
}
