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
            strct: {
                tupl: (1, {
                    a: [(1,{
                        b: 'C'
                    },[1,2,3]); 2]

                })
            },
        }
    }
);

#[test]
fn happy() {
    assert_eq!(Language::DE.happy().strct().tupl().0(), 1);
    assert_eq!(Language::DE.happy().strct().tupl().1().a()[0].0(), 1);
    assert_eq!(Language::DE.happy().strct().tupl().1().a()[0].1().b(), 'C');
    assert_eq!(
        Language::DE.happy().strct().tupl().1().a()[0].2(),
        [1, 2, 3]
    );
}
