#![feature(test)]

extern crate test;
use test::Bencher;

use comfy_i18n_macro::{i18n, i18n_init};

i18n_init!(
    #[fallback]
    DE,
    EN,
);

i18n!(
    benchmark,
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
        fmt7_with_specifier: "{:^20}{:^20}{:^20}{:^20}{:^20}{:^20}{:^20}",
    },
    EN: {
        simple_value: "EN simple value",
        simple_value_flt: 3.15,
    }
);

#[bench]
fn simple_value_str(b: &mut Bencher) {
    b.iter(|| I18n::DE.benchmark().simple_value());
}

#[bench]
fn simple_value_flt(b: &mut Bencher) {
    b.iter(|| I18n::DE.benchmark().simple_value_flt());
}

#[bench]
fn simple_value_nested(b: &mut Bencher) {
    b.iter(|| {
        I18n::DE
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
    b.iter(|| I18n::DE.benchmark().fmt2(&"Test1", &"Test2"));
}

#[bench]
fn fmt2_complex(b: &mut Bencher) {
    b.iter(|| I18n::DE.benchmark().fmt2(&3.14f64, &6.28f64));
}

#[bench]
fn fmt2_complex_with_dynamic_args(b: &mut Bencher) {
    let args = [1.14f64, 2.14f64];
    b.iter(|| I18n::DE.benchmark().fmt2(&args[0], &args[1]));
}

#[bench]
fn fmt2_complex_with_dynamic_args_and_specifier(b: &mut Bencher) {
    let args = [1.14f64, 2.14f64];
    b.iter(|| I18n::DE.benchmark().fmt2(&args[0], &args[1]));
}

#[bench]
fn fmt7_simple(b: &mut Bencher) {
    b.iter(|| {
        I18n::DE.benchmark().fmt7(
            &"Test1", &"Test2", &"Test3", &"Test4", &"Test5", &"Test6", &"Test7",
        )
    });
}

#[bench]
fn fmt7_complex(b: &mut Bencher) {
    b.iter(|| {
        I18n::DE.benchmark().fmt7(
            &1.14f64, &2.14f64, &3.14f64, &4.14f64, &5.14f64, &6.14f64, &7.14f64,
        )
    });
}

#[bench]
fn fmt7_complex_with_dynamic_args(b: &mut Bencher) {
    let args = [
        1.14f64, 2.14f64, 3.14f64, 4.14f64, 5.14f64, 6.14f64, 7.14f64,
    ];
    b.iter(|| {
        I18n::DE.benchmark().fmt7(
            &args[0], &args[1], &args[2], &args[3], &args[4], &args[5], &args[6],
        )
    });
}

#[bench]
fn fmt7_complex_with_dynamic_args_and_specifier(b: &mut Bencher) {
    let args = [
        1.14f64, 2.14f64, 3.14f64, 4.14f64, 5.14f64, 6.14f64, 7.14f64,
    ];
    b.iter(|| {
        I18n::DE.benchmark().fmt7(
            &args[0], &args[1], &args[2], &args[3], &args[4], &args[5], &args[6],
        )
    });
}
