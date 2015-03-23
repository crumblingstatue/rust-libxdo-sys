extern crate "libxdo-sys" as libxdo;

use libxdo::*;
use std::ptr::null;

fn main() {
    unsafe {
        let xdo = xdo_new(null());

        if xdo.is_null() {
            panic!("Failed to init libxdo.");
        }

        if xdo_mousedown(xdo, CURRENTWINDOW, 1) != 0 {
            panic!("Failed to hold mouse down.");
        }

        if xdo_mousemove_relative(xdo, 150, 150) != 0 {
            panic!("Failed to move mouse.");
        }

        if xdo_mouseup(xdo, CURRENTWINDOW, 1) != 0 {
            panic!("Failed to release mouse.");
        }

        xdo_free(xdo);
    }
}
