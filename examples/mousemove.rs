extern crate "libxdo-sys" as libxdo;

use libxdo::*;
use std::ptr::null_mut;

fn main() {
    unsafe {
        let xdo = xdo_new(null_mut());

        if xdo.is_null() {
            panic!("Failed to init libxdo.");
        }

        xdo_mousemove(xdo, 0, 0, 0);
        xdo_free(xdo);
    }
}
