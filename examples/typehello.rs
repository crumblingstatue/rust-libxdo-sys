extern crate libxdo_sys;

use libxdo_sys::*;
use std::ptr::null;

fn main() {
    unsafe {
        let xdo = xdo_new(null());

        if xdo.is_null() {
            panic!("Failed to init libxdo.");
        }

        if xdo_enter_text_window(xdo, CURRENTWINDOW, "Hello, World!\0".as_ptr() as *const i8,
                                 250000) != 0 {
            panic!("Failed to move mouse.");
        }

        xdo_free(xdo);
    }
}
