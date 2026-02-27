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
fn fmt7_simple(b: &mut Bencher) {
    b.iter(|| {
        t!(
            "fmt2",
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
            "fmt2",
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
