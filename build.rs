extern crate gcc;

fn main() {
    gcc::Config::new().file("src/callbacks.c")
                      .include("src")
                      .compile("libcallbacks.a");

   // seem unable to compile the ext/pb_ds based map :(
   // gcc::Config::new().file("src/stdcc_ccmap.cpp")
   //                   // OSX-only - force the GNU+11 standard
   //                   // still not enough as doesn't contain ext/pb_ds
   //                   //.flag("-stdlib=libc++")
   //                   //.flag("-std=gnu++11")
   //                   .flag("-std=c++11")
   //                   .include("src")
   //                   .compile("libstdcc_ccmap.a");
}
