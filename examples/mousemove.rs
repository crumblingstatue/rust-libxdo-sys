use libxdo_sys::*;
use std::ptr::null;

fn main() {
    unsafe {
        let xdo = xdo_new(null());

        if xdo.is_null() {
            panic!("Failed to init libxdo.");
        }

        if xdo_move_mouse(xdo, 0, 0, 0) != 0 {
            panic!("Failed to move mouse.");
        }
        xdo_free(xdo);
    }
}
