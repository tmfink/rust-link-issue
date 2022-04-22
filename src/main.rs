#![allow(unused_imports, dead_code)]

use std::os::raw::{c_char, c_int};

extern "C" {
    fn foo() -> usize;

    fn kvm_open(
        execfile: *const c_char,
        corefile: *const c_char,
        swapfile: *mut c_char,
        flags: c_int,
        errstr: *const c_char,
    );
}

fn main() {
    unsafe {
        dbg!(foo());
    }

    // Uncomment to workaround link issue
    //println!("kvm_open() = {:?}", kvm_open as *const u8);
}
