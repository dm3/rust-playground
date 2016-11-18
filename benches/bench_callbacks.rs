#![feature(test)]

extern crate test;
extern crate playground;

use playground::callbacks::{run, extern_inc, run_with_closure, run_extern_with_closure};
use std::ptr;
use self::test::Bencher;

#[bench]
fn bench_extern(b: &mut Bencher) {
    unsafe {
        b.iter(|| run(0, ptr::null_mut(), extern_inc));
    }
}

#[bench]
fn bench_closure(b: &mut Bencher) {
    b.iter(|| run_with_closure(0, |v| v + 1));
}

#[bench]
fn bench_closure_extern(b: &mut Bencher) {
    b.iter(|| run_extern_with_closure(0, |v| v + 1));
}
