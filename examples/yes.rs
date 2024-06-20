use libxdo_sys::*;
use std::ptr::null;
use std::time::Duration;

fn main() {
    unsafe {
        let xdo = xdo_new(null());

        if xdo.is_null() {
            panic!("Failed to init libxdo.");
        }

        for _ in 0..10 {
            if xdo_send_keysequence_window(xdo, CURRENTWINDOW, "y\0".as_ptr() as *const i8, 0) != 0
            {
                panic!("Couldn't press `y`.");
            }
            if xdo_send_keysequence_window(xdo, CURRENTWINDOW, "Return\0".as_ptr() as *const i8, 0)
                != 0
            {
                panic!("Couldn't press `return`.");
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        xdo_free(xdo);
    }
}
