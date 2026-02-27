#![feature(test)]

extern crate test;
use test::Bencher;

use comfy_i18n_macro::{ComfyI18n, i18n};

#[derive(ComfyI18n)]
pub enum Language {
    #[fallback]
    DE,
    EN,
}

i18n!(
    name: Benchmark,
    key: crate::Language,
    translations: {
        DE: {
            simple_value: "DE simple value",
            simple_value_flt: 3.14,
            nested1: {
                nested2: {
                    nested3: {
                        nested4: {
                            simple_nested_value: "Wambo"
                        }
                    }
                }
            },
            fmt2: "{}{}",
            fmt7: "{}{}{}{}{}{}{}",
        },
        EN: {
            simple_value: "EN simple value",
            simple_value_flt: 3.15,
        }
    }
);

#[bench]
fn simple_value_str(b: &mut Bencher) {
    b.iter(|| Language::DE.benchmark().simple_value());
}

#[bench]
fn simple_value_flt(b: &mut Bencher) {
    b.iter(|| Language::DE.benchmark().simple_value_flt());
}

#[bench]
fn simple_value_nested(b: &mut Bencher) {
    b.iter(|| {
        Language::DE
            .benchmark()
            .nested1()
            .nested2()
            .nested3()
            .nested4()
            .simple_nested_value()
    });
}

#[bench]
fn fmt2_simple(b: &mut Bencher) {
    b.iter(|| Language::DE.benchmark().fmt2(&"Test1", &"Test2"));
}

#[bench]
fn fmt2_complex(b: &mut Bencher) {
    b.iter(|| Language::DE.benchmark().fmt2(&3.14f64, &6.28f64));
}

#[bench]
fn fmt7_simple(b: &mut Bencher) {
    b.iter(|| {
        Language::DE.benchmark().fmt7(
            &"Test1", &"Test2", &"Test3", &"Test4", &"Test5", &"Test6", &"Test7",
        )
    });
}

#[bench]
fn fmt7_complex(b: &mut Bencher) {
    b.iter(|| {
        Language::DE.benchmark().fmt7(
            &1.14f64, &2.14f64, &3.14f64, &4.14f64, &5.14f64, &6.14f64, &7.14f64,
        )
    });
}
