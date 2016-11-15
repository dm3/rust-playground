use libc::{c_int, c_void};

#[link(name = "callbacks")]
extern "C" {
    pub fn run(val: i32,
               arg: *mut c_void,
               callback: extern "C" fn(c_int, *mut c_void) -> c_int)
               -> c_int;
}

pub extern "C" fn extern_inc(val: c_int, _: *mut c_void) -> c_int {
    ((val as i32) + 1) as c_int
}

pub fn run_with_closure<F>(val: i32, f: F) -> i32
    where F: Fn(i32) -> i32
{
    f(val)
}

extern "C" fn run_extern_with_closure_handler<F>(val: c_int, cb: *mut c_void) -> c_int
    where F: Fn(i32) -> i32
{

    unsafe {
        let closure = (cb as *const F).as_ref();
        let res = closure.unwrap()(val as i32);
        return res as c_int;
    }
}

pub fn run_extern_with_closure<F>(val: i32, f: F) -> i32
    where F: Fn(i32) -> i32
{
    let cb = &f as *const _ as *mut c_void;
    unsafe { run(val, cb, run_extern_with_closure_handler::<F>) }
}
