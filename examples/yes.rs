#![feature(std_misc, thread_sleep)]

extern crate "libxdo-sys" as libxdo;

use libxdo::*;
use std::ptr::null;
use std::time::Duration;

fn main() {
    unsafe {
        let xdo = xdo_new(null());

        if xdo.is_null() {
            panic!("Failed to init libxdo.");
        }

        for _ in 0..10 {
            if xdo_keysequence(xdo, CURRENTWINDOW, "y\0".as_ptr() as *const i8, 0) != 0 {
                panic!("Couldn't press `y`.");
            }
            if xdo_keysequence(xdo, CURRENTWINDOW, "Return\0".as_ptr() as *const i8, 0) != 0 {
                panic!("Couldn't press `return`.");
            }
            std::thread::sleep(Duration::milliseconds(100));
        }

        xdo_free(xdo);
    }
}