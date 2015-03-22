#![allow(non_camel_case_types)]

extern crate x11;
extern crate libc;

use x11::xlib::{
    Display,
    KeyCode,
    KeySym,
    XModifierKeymap,
};

use libc::{
    c_int,
    c_char,
    wchar_t,
};

#[repr(C)]
pub struct charcodemap {
    pub key: wchar_t,
    pub code: KeyCode,
    pub symbol: KeySym,
    index: c_int,
    modmask: c_int,
    needs_binding: c_int,
}

#[repr(C)]
pub struct xdo {
    pub xdpy: *mut Display,
    pub display_name: *mut c_char,
    pub charcodes: *mut charcodemap,
    pub charcodes_len: c_int,
    pub modmap: *mut XModifierKeymap,
    pub keymap: *mut KeySym,
    pub keycode_high: c_int,
    pub keycode_low: c_int,
    pub keysyms_per_keycode: c_int,
    pub close_display_when_freed: c_int,
    pub quiet: c_int,
    pub debug: c_int,
    pub features_mask: c_int,
}

extern "C" {
    pub fn xdo_new(display: *const c_char) -> *mut xdo;
    pub fn xdo_free(xdo: *mut xdo);
    pub fn xdo_mousemove(xdo: *const xdo, x: c_int, y: c_int, screen: c_int);
}
