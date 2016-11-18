extern crate playground;
extern crate env_logger;
extern crate ws;

use std::thread;
use playground::ws_test;

fn start_client(addr: &'static str, name: &'static str) {
    ws::connect(addr, |out| {
        out.send("hi!").unwrap();
        move |msg| {
            println!("Client {} got msg {}!", name, msg);
            Ok(())
        }
    }).expect("Could not connect!");
}

fn main() {
    env_logger::init().unwrap();
    println!("> starting up...");
    ws_test::start("127.0.0.1:3100", 0);
    thread::sleep_ms(100);
    println!("> started threads...");
    let a = thread::spawn(move || { start_client("ws://127.0.0.1:3100", "A") });
    println!("> started client A...");
    thread::sleep_ms(5000);
    let b = thread::spawn(move || { start_client("ws://127.0.0.1:3100", "B") });
    println!("> started client B...");
    a.join();
    b.join();
}
