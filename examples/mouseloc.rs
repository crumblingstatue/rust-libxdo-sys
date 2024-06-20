use std::ptr::null;
use {libxdo_sys::*, x11::xlib::Window};

fn main() {
    unsafe {
        let xdo = xdo_new(null());

        if xdo.is_null() {
            panic!("Failed to init libxdo.");
        }

        let mut x = 0;
        let mut y = 0;
        let mut screen = 0;
        let mut window: Window = 0;
        xdo_get_mouse_location2(xdo, &mut x, &mut y, &mut screen, &mut window);
        eprintln!("{x}, {y}, {screen}, {window}");

        xdo_free(xdo);
    }
}
