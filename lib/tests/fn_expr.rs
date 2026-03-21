use comfy_i18n_macro::{i18n, i18n_init};

i18n_init!(
    #[fallback]
    DE,
    EN,
);

i18n!(
    happy,
    DE: {
        fn_expr: |&self, truth: &i32| -> String {
            format!("Die Wahrheit ist: {}", truth)
        },
    },
    EN: {
        fn_expr: |&self, truth: &i32| -> String {
            format!("The truth is: {}", truth)
        },
    }
);

#[test]
fn happy() {
    assert_eq!(I18n::DE.happy().fn_expr(&42), "Die Wahrheit ist: 42");
    assert_eq!(I18n::EN.happy().fn_expr(&42), "The truth is: 42");
}
