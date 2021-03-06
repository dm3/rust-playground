#![feature(proc_macro)]
#![feature(rand)]

extern crate rand;
extern crate libc;

// extern crate serde;
// extern crate serde_json;
// #[macro_use]
// extern crate serde_derive;

extern crate futures;

#[macro_use]
extern crate log;

extern crate ws;

pub mod callbacks;
pub mod ws_test;
pub mod maps;
