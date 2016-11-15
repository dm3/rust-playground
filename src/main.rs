extern crate playground;
extern crate env_logger;
extern crate ws;

use std::thread;
use playground::ws_test;

fn main() {
    env_logger::init().unwrap();
    println!("> starting up...");
    ws_test::start("127.0.0.1:3100", 0);
    thread::sleep_ms(100);
    println!("> started threads...");
    ws::connect("ws://127.0.0.1:3100", |out| {
        out.send("hi!").unwrap();
        move |msg| {
            println!("Client got msg {}!", msg);
            Ok(())
        }
    }).expect("Could not connect!");
}
