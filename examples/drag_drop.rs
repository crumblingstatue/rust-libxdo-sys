extern crate libxdo_sys;

use libxdo_sys::*;
use std::ptr::null;

fn main() {
    unsafe {
        let xdo = xdo_new(null());

        if xdo.is_null() {
            panic!("Failed to init libxdo.");
        }

        if xdo_mouse_down(xdo, CURRENTWINDOW, 1) != 0 {
            panic!("Failed to hold mouse down.");
        }

        if xdo_move_mouse_relative(xdo, 150, 150) != 0 {
            panic!("Failed to move mouse.");
        }

        if xdo_mouse_up(xdo, CURRENTWINDOW, 1) != 0 {
            panic!("Failed to release mouse.");
        }

        xdo_free(xdo);
    }
}
