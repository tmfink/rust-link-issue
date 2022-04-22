#![allow(unused_imports, dead_code)]

use std::os::raw::{c_char, c_int};

extern "C" {
    fn foo() -> usize;

    fn zlibVersion() -> *const c_char;
}

fn main() {
    unsafe {
        println!("{:x}", foo());
    }

    // Uncomment to workaround link issue
    //println!("zlibVersion() = {:?}", zlibVersion as *const u8);
}
