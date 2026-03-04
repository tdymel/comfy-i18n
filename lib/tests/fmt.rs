use comfy_i18n_macro::{i18n, i18n_init};

i18n_init!(
    #[fallback]
    DE,
    EN,
);

i18n!(
    name: Happy,
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
    assert_eq!(I18n::DE.happy().value(&"Welt"), "Hallo, Welt!");

    assert_eq!(I18n::EN.happy().value(&"World"), "Hello, World!");
}

i18n!(
    name: FallbackTest,
    translations: {
        DE: {
            value: "Hallo, {world}!",
        },
    }
);

#[test]
fn fallback() {
    assert_eq!(I18n::DE.fallback_test().value(&"Welt"), "Hallo, Welt!");

    assert_eq!(I18n::EN.fallback_test().value(&"World"), "Hallo, World!");
}

#[test]
fn fmt2() {
    assert_eq!(I18n::DE.happy().fmt2(&"Test1", &"Test2"), "Test1 Test2");
}

#[test]
fn fmt7() {
    assert_eq!(
        I18n::DE.happy().fmt7(
            &"Test1", &"Test2", &"Test3", &"Test4", &"Test5", &"Test6", &"Test7"
        ),
        "Test1 Test2 Test3 Test4 Test5 Test6 Test7"
    );
}

i18n!(
    name: SelfReferences,
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
    assert_eq!(I18n::DE.self_references().nested().value(), "Hallo, Welt!");

    assert_eq!(I18n::DE.self_references().nested().value2(), "Hallo, Welt!");
}
