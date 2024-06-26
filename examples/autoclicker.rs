use libxdo_sys::*;
use std::ptr::null;
use std::time::Duration;

fn main() {
    unsafe {
        let xdo = xdo_new(null());

        if xdo.is_null() {
            panic!("Failed to init libxdo.");
        }

        for _ in 0..5 {
            if xdo_click_window(xdo, CURRENTWINDOW, 1) != 0 {
                panic!("Couldn't click!");
            }
            std::thread::sleep(Duration::from_millis(1000));
        }

        xdo_free(xdo);
    }
}
