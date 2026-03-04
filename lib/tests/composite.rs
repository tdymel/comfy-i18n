use comfy_i18n_macro::{i18n, i18n_init};

i18n_init!(
    #[fallback]
    DE,
    EN,
);

i18n!(
    happy,
    DE: {
        strct: {
            tupl: (1, {
                a: [(1,{
                    b: 'C'
                },[1,2,3]); 2]

            })
        },
    }
);

#[test]
fn happy() {
    assert_eq!(I18n::DE.happy().strct().tupl().0(), &1);
    assert_eq!(I18n::DE.happy().strct().tupl().1().a()[0].0(), &1);
    assert_eq!(I18n::DE.happy().strct().tupl().1().a()[0].1().b(), &'C');
    assert_eq!(
        I18n::DE.happy().strct().tupl().1().a()[0].2().value(),
        &[1, 2, 3]
    );
}
