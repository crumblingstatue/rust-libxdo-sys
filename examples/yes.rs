extern crate libxdo_sys;

use libxdo_sys::*;
use std::ptr::null;

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
            std::thread::sleep_ms(100);
        }

        xdo_free(xdo);
    }
}
