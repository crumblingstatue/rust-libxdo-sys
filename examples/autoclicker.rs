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

        for _ in 0..5 {
            if xdo_click(xdo, CURRENTWINDOW, 1) != 0 {
                panic!("Couldn't click!");
            }
            std::thread::sleep(Duration::seconds(1));
        }

        xdo_free(xdo);
    }
}
