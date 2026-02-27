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
            fmt2: "{arg1} {arg2}",
            fmt7: "{arg1} {arg2} {arg3} {arg4} {arg5} {arg6} {arg7}",
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

    assert_eq!(
        Language::EN.fallback_test().value(&"World"),
        "Hallo, World!"
    );
}

#[test]
fn fmt2() {
    assert_eq!(Language::DE.happy().fmt2(&"Test1", &"Test2"), "Test1 Test2");
}

#[test]
fn fmt7() {
    assert_eq!(
        Language::DE.happy().fmt7(
            &"Test1", &"Test2", &"Test3", &"Test4", &"Test5", &"Test6", &"Test7"
        ),
        "Test1 Test2 Test3 Test4 Test5 Test6 Test7"
    );
}

i18n!(
    name: SelfReferences,
    key: crate::Language,
    translations: {
        DE: {
            nested: {
                value: "Hallo, {self.world}!",
                value2: "Hallo, {root.nested.world}!",
                world: "Welt"
            }
        },
    }
);

#[test]
fn self_ref() {
    assert_eq!(
        Language::DE.self_references().nested().value(),
        "Hallo, Welt!"
    );

    assert_eq!(
        Language::DE.self_references().nested().value2(),
        "Hallo, Welt!"
    );
}
