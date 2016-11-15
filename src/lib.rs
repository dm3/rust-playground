#![feature(proc_macro)]

extern crate libc;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

#[macro_use]
extern crate log;

extern crate mio;
extern crate ws;

pub mod callbacks;
pub mod tokio;
pub mod ws_test;
