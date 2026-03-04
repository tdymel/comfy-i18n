#![feature(test)]

extern crate test;
use test::Bencher;

use rust_i18n::t;

rust_i18n::i18n!("locales", fallback = "en");

#[bench]
fn simple_value_str(b: &mut Bencher) {
    b.iter(|| t!("simple_value"));
}

#[bench]
fn simple_value_flt(b: &mut Bencher) {
    b.iter(|| t!("simple_value_flt"));
}

#[bench]
fn simple_value_nested(b: &mut Bencher) {
    b.iter(|| t!("nested1.nested2.nested3.nested4.simple_nested_value"));
}

#[bench]
fn fmt2_simple(b: &mut Bencher) {
    b.iter(|| t!("fmt2", a = "Test1", b = "Test2"));
}

#[bench]
fn fmt2_complex(b: &mut Bencher) {
    b.iter(|| t!("fmt2", a = 3.14f64, b = 6.28f64));
}

#[bench]
fn fmt2_complex_dynamic_with_args(b: &mut Bencher) {
    let args = [1.14f64, 2.14f64];
    b.iter(|| t!("fmt2", a = args[0], b = args[1],));
}

#[bench]
fn fmt2_complex_with_dynamic_args_and_specifier(b: &mut Bencher) {
    let args = [1.14f64, 2.14f64];
    b.iter(|| {
        t!(
            "fmt2",
            a = args[0] : {:#^20},
            b = args[1] : {:#^20},
        )
    });
}

#[bench]
fn fmt7_simple(b: &mut Bencher) {
    b.iter(|| {
        t!(
            "fmt7",
            a = "Test1",
            b = "Test2",
            c = "Test3",
            d = "Test4",
            e = "Test5",
            f = "Test6",
            g = "Test7"
        )
    });
}

#[bench]
fn fmt7_complex(b: &mut Bencher) {
    b.iter(|| {
        t!(
            "fmt7",
            a = 1.14f64,
            b = 2.14f64,
            c = 3.14f64,
            d = 4.14f64,
            e = 5.14f64,
            f = 6.14f64,
            g = 7.14f64,
        )
    });
}

#[bench]
fn fmt7_complex_dynamic_with_args(b: &mut Bencher) {
    let args = [
        1.14f64, 2.14f64, 3.14f64, 4.14f64, 5.14f64, 6.14f64, 7.14f64,
    ];
    b.iter(|| {
        t!(
            "fmt7",
            a = args[0],
            b = args[1],
            c = args[2],
            d = args[3],
            e = args[4],
            f = args[5],
            g = args[6]
        )
    });
}

#[bench]
fn fmt7_complex_with_dynamic_args_and_specifier(b: &mut Bencher) {
    let args = [
        1.14f64, 2.14f64, 3.14f64, 4.14f64, 5.14f64, 6.14f64, 7.14f64,
    ];
    b.iter(|| {
        t!(
            "fmt7",
            a = args[0] : {:#^20},
            b = args[1] : {:#^20},
            c = args[2] : {:#^20},
            d = args[3] : {:#^20},
            e = args[4] : {:#^20},
            f = args[5] : {:#^20},
            g = args[6] : {:#^20}
        )
    });
}
