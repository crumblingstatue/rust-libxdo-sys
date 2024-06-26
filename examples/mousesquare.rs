use libxdo_sys::*;
use std::ptr::null;
use std::time::Duration;

unsafe fn move_it(xdo: *const xdo_t, rel_x: i32, rel_y: i32, times: i32) {
    for _ in 0..times {
        if xdo_move_mouse_relative(xdo, rel_x, rel_y) != 0 {
            panic!("Failed to move mouse.");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    unsafe {
        let xdo = xdo_new(null());

        if xdo.is_null() {
            panic!("Failed to init libxdo.");
        }

        move_it(xdo, 10, 0, 20);
        move_it(xdo, 0, 10, 20);
        move_it(xdo, -10, 0, 20);
        move_it(xdo, 0, -10, 20);

        xdo_free(xdo);
    }
}
