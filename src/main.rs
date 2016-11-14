#![feature(test)]

extern crate libc;

use std::{ptr, mem};
use libc::{c_int, c_void};

#[link(name = "main")]
extern "C" {
    fn run(val: i32,
           arg: *mut c_void,
           callback: extern fn(c_int, *mut c_void) -> c_int)
           -> c_int;
}

extern "C" fn extern_inc(val: c_int, _: *mut c_void) -> c_int {
    ((val as i32) + 1) as c_int
}

pub fn run_with_closure<F>(val: i32, f: F) -> i32 where F: Fn(i32) -> i32 {
    f(val)
}

extern "C" fn run_extern_with_closure_handler<F>(val: c_int, cb: *mut c_void) -> c_int
    where F: Fn(i32) -> i32 {

    let closure = cb as *mut Option<F>;
    unsafe {
        let res = (*closure).take().unwrap()(val as i32);
        return res as c_int;
    }
}

pub fn run_extern_with_closure<F>(val: i32, f: F) -> i32 where F: Fn(i32) -> i32 {
    let cb = &f as *const _ as *mut c_void;
    unsafe {
       run(val, cb, run_extern_with_closure_handler::<F>)
    }
}

fn main() {
    unsafe {
        println!("{}", run(0, ptr::null_mut(), extern_inc));
        println!("{}", run_extern_with_closure(0, |v| v + 1));
    }
    ()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::ptr;
    use self::test::Bencher;

    #[test]
    fn extern_works() {
        unsafe {
            assert_eq!(1, super::run(0, ptr::null_mut(), super::extern_inc));
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
}

