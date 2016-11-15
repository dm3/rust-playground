extern crate gcc;

fn main() {
    gcc::Config::new().file("src/callbacks.c")
                      .include("src")
                      .compile("libcallbacks.a");
}
