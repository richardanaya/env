#![no_std]
extern crate libw;
use libw::*;

#[no_mangle]
pub fn _start() {
    let vars = environment_variables();
    for v in vars {
        print(&v.name);
        print("=");
        println(&v.value);
    }
}
