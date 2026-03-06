use comfy_i18n_macro::{i18n, i18n_init};

i18n_init!(
    #[fallback]
    DE,
    EN,
);

i18n!(
    happy,
    DE: {
        value: "Hallo, {world}!",
        fmt2: "{arg1} {arg2}",
        fmt7: "{arg1} {arg2} {arg3} {arg4} {arg5} {arg6} {arg7}",
    },
    EN: {
        value: "Hello, {world}!"
    }
);

#[test]
fn happy() {
    assert_eq!(I18n::DE.happy().value(&"Welt"), "Hallo, Welt!");

    assert_eq!(I18n::EN.happy().value(&"World"), "Hello, World!");
}

i18n!(
    fallback_test,
    DE: {
        value: "Hallo, {world}!",
    },
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
    references,
    DE: {
        other_self: "{self.nested.world}",
        context_ref: "{context.references.nested.world}",
        i18n_ref: "{i18n.EN.references.nested.world}",
        nested: {
            value: "Hallo, {self.world}!",
            value2: "Hallo, {root.nested.world}!",
            world: "Welt"
        }
    },
    EN: {
        hello: "Hello"
    }
);

#[test]
fn self_ref() {
    assert_eq!(I18n::DE.references().nested().value(), "Hallo, Welt!");

    assert_eq!(I18n::DE.references().nested().value2(), "Hallo, Welt!");
    assert_eq!(I18n::DE.references().other_self(), "Welt");
    assert_eq!(I18n::DE.references().context_ref(), "Welt");
    assert_eq!(I18n::DE.references().i18n_ref(), "Welt");
}
