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
            i8_value: 1i8,
            i16_value: 1i16,
            i32_value: 1i32,
            i64_value: 1i64,
            i128_value: 1i128,
            u8_value: 1u8,
            u16_value: 1u16,
            u32_value: 1u32,
            u64_value: 1u64,
            u128_value: 1u128,
            usize_value: 1usize,
        },
        EN: {
            i8_value: 2i8,
            i16_value: 2i16,
            i32_value: 2i32,
            i64_value: 2i64,
            i128_value: 2i128,
            u8_value: 2u8,
            u16_value: 2u16,
            u32_value: 2u32,
            u64_value: 2u64,
            u128_value: 2u128,
            usize_value: 2usize,
        }
    }
);

#[test]
fn happy() {
    assert_eq!(Language::DE.happy().i8_value(), &1i8);
    assert_eq!(Language::DE.happy().i16_value(), &1i16);
    assert_eq!(Language::DE.happy().i32_value(), &1i32);
    assert_eq!(Language::DE.happy().i64_value(), &1i64);
    assert_eq!(Language::DE.happy().i128_value(), &1i128);
    assert_eq!(Language::DE.happy().u8_value(), &1u8);
    assert_eq!(Language::DE.happy().u16_value(), &1u16);
    assert_eq!(Language::DE.happy().u32_value(), &1u32);
    assert_eq!(Language::DE.happy().u64_value(), &1u64);
    assert_eq!(Language::DE.happy().u128_value(), &1u128);
    assert_eq!(Language::DE.happy().usize_value(), &1i32); // TODO

    assert_eq!(Language::EN.happy().i8_value(), &2i8);
    assert_eq!(Language::EN.happy().i16_value(), &2i16);
    assert_eq!(Language::EN.happy().i32_value(), &2i32);
    assert_eq!(Language::EN.happy().i64_value(), &2i64);
    assert_eq!(Language::EN.happy().i128_value(), &2i128);
    assert_eq!(Language::EN.happy().u8_value(), &2u8);
    assert_eq!(Language::EN.happy().u16_value(), &2u16);
    assert_eq!(Language::EN.happy().u32_value(), &2u32);
    assert_eq!(Language::EN.happy().u64_value(), &2u64);
    assert_eq!(Language::EN.happy().u128_value(), &2u128);
    assert_eq!(Language::EN.happy().usize_value(), &2i32); // TODO
}

i18n!(
    name: FallbackTest,
    key: crate::Language,
    translations: {
        DE: {
            i8_value: 1i8,
        },
    }
);

#[test]
fn fallback() {
    assert_eq!(
        Language::DE.fallback_test().i8_value(),
        Language::EN.fallback_test().i8_value()
    );
}
