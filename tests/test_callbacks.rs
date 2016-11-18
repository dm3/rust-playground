#![feature(test)]

extern crate test;
extern crate playground;

use playground::callbacks::{run, extern_inc, run_with_closure, run_extern_with_closure};
use std::ptr;

#[test]
fn extern_works() {
    unsafe {
        assert_eq!(1, run(0, ptr::null_mut(), extern_inc));
    }
}

#[test]
fn closure_works() {
    assert_eq!(2, run_with_closure(1, |v| v + 1));
}

#[test]
fn closure_extern_works() {
    assert_eq!(3, run_extern_with_closure(2, |v| v + 1));
}

