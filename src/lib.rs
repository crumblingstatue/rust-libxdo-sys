#![allow(non_camel_case_types)]

use {
    libc::{useconds_t, wchar_t},
    x11::xlib::{Atom, Display, KeyCode, KeySym, Screen, Window, XID},
};

pub const CURRENTWINDOW: XID = 0;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_charcodemap {
    pub key: wchar_t,
    pub code: KeyCode,
    pub symbol: KeySym,
    pub group: ::libc::c_int,
    pub modmask: ::libc::c_int,
    pub needs_binding: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_charcodemap {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_charcodemap {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type charcodemap_t = Struct_charcodemap;
type Enum_Unnamed1 = ::libc::c_uint;
pub const XDO_FEATURE_XTEST: ::libc::c_uint = 0;
pub type XDO_FEATURES = Enum_Unnamed1;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_xdo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
impl ::std::clone::Clone for Struct_xdo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_xdo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
type Enum_Unnamed2 = ::libc::c_uint;
pub const SEARCH_ANY: libc::c_uint = 0;
pub const SEARCH_ALL: libc::c_uint = 1;
pub type xdo_t = Struct_xdo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_xdo_search {
    pub title: *const ::libc::c_char,
    pub winclass: *const ::libc::c_char,
    pub winclassname: *const ::libc::c_char,
    pub winname: *const ::libc::c_char,
    pub pid: ::libc::c_int,
    pub max_depth: ::libc::c_long,
    pub only_visible: ::libc::c_int,
    pub screen: ::libc::c_int,
    pub require: Enum_Unnamed2,
    pub searchmask: ::libc::c_uint,
    pub desktop: ::libc::c_long,
    pub limit: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_xdo_search {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_xdo_search {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type xdo_search_t = Struct_xdo_search;
extern "C" {
    pub fn xdo_new(display: *const ::libc::c_char) -> *mut xdo_t;
    pub fn xdo_new_with_opened_display(
        xdpy: *mut Display,
        display: *const ::libc::c_char,
        close_display_when_freed: ::libc::c_int,
    ) -> *mut xdo_t;
    pub fn xdo_version() -> *const ::libc::c_char;
    pub fn xdo_free(xdo: *mut xdo_t);
    pub fn xdo_move_mouse(
        xdo: *const xdo_t,
        x: ::libc::c_int,
        y: ::libc::c_int,
        screen: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_move_mouse_relative_to_window(
        xdo: *const xdo_t,
        window: Window,
        x: ::libc::c_int,
        y: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_move_mouse_relative(
        xdo: *const xdo_t,
        x: ::libc::c_int,
        y: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_mouse_down(
        xdo: *const xdo_t,
        window: Window,
        button: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_mouse_up(xdo: *const xdo_t, window: Window, button: ::libc::c_int) -> ::libc::c_int;
    pub fn xdo_get_mouse_location(
        xdo: *const xdo_t,
        x: *mut ::libc::c_int,
        y: *mut ::libc::c_int,
        screen_num: *mut ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_get_window_at_mouse(xdo: *const xdo_t, window_ret: *mut Window) -> ::libc::c_int;
    pub fn xdo_get_mouse_location2(
        xdo: *const xdo_t,
        x_ret: *mut ::libc::c_int,
        y_ret: *mut ::libc::c_int,
        screen_num_ret: *mut ::libc::c_int,
        window_ret: *mut Window,
    ) -> ::libc::c_int;
    pub fn xdo_wait_for_mouse_move_from(
        xdo: *const xdo_t,
        origin_x: ::libc::c_int,
        origin_y: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_wait_for_mouse_move_to(
        xdo: *const xdo_t,
        dest_x: ::libc::c_int,
        dest_y: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_click_window(
        xdo: *const xdo_t,
        window: Window,
        button: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_click_window_multiple(
        xdo: *const xdo_t,
        window: Window,
        button: ::libc::c_int,
        repeat: ::libc::c_int,
        delay: useconds_t,
    ) -> ::libc::c_int;
    pub fn xdo_enter_text_window(
        xdo: *const xdo_t,
        window: Window,
        string: *const ::libc::c_char,
        delay: useconds_t,
    ) -> ::libc::c_int;
    pub fn xdo_send_keysequence_window(
        xdo: *const xdo_t,
        window: Window,
        keysequence: *const ::libc::c_char,
        delay: useconds_t,
    ) -> ::libc::c_int;
    pub fn xdo_send_keysequence_window_up(
        xdo: *const xdo_t,
        window: Window,
        keysequence: *const ::libc::c_char,
        delay: useconds_t,
    ) -> ::libc::c_int;
    pub fn xdo_send_keysequence_window_down(
        xdo: *const xdo_t,
        window: Window,
        keysequence: *const ::libc::c_char,
        delay: useconds_t,
    ) -> ::libc::c_int;
    pub fn xdo_send_keysequence_window_list_do(
        xdo: *const xdo_t,
        window: Window,
        keys: *mut charcodemap_t,
        nkeys: ::libc::c_int,
        pressed: ::libc::c_int,
        modifier: *mut ::libc::c_int,
        delay: useconds_t,
    ) -> ::libc::c_int;
    pub fn xdo_get_active_keys_to_keycode_list(
        xdo: *const xdo_t,
        keys: *mut *mut charcodemap_t,
        nkeys: *mut ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_wait_for_window_map_state(
        xdo: *const xdo_t,
        wid: Window,
        map_state: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_wait_for_window_size(
        xdo: *const xdo_t,
        window: Window,
        width: ::libc::c_uint,
        height: ::libc::c_uint,
        flags: ::libc::c_int,
        to_or_from: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_move_window(
        xdo: *const xdo_t,
        wid: Window,
        x: ::libc::c_int,
        y: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_translate_window_with_sizehint(
        xdo: *const xdo_t,
        window: Window,
        width: ::libc::c_uint,
        height: ::libc::c_uint,
        width_ret: *mut ::libc::c_uint,
        height_ret: *mut ::libc::c_uint,
    ) -> ::libc::c_int;
    pub fn xdo_set_window_size(
        xdo: *const xdo_t,
        wid: Window,
        w: ::libc::c_int,
        h: ::libc::c_int,
        flags: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_set_window_property(
        xdo: *const xdo_t,
        wid: Window,
        property: *const ::libc::c_char,
        value: *const ::libc::c_char,
    ) -> ::libc::c_int;
    pub fn xdo_set_window_class(
        xdo: *const xdo_t,
        wid: Window,
        name: *const ::libc::c_char,
        _class: *const ::libc::c_char,
    ) -> ::libc::c_int;
    pub fn xdo_set_window_urgency(
        xdo: *const xdo_t,
        wid: Window,
        urgency: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_set_window_override_redirect(
        xdo: *const xdo_t,
        wid: Window,
        override_redirect: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_focus_window(xdo: *const xdo_t, wid: Window) -> ::libc::c_int;
    pub fn xdo_raise_window(xdo: *const xdo_t, wid: Window) -> ::libc::c_int;
    pub fn xdo_get_focused_window(xdo: *const xdo_t, window_ret: *mut Window) -> ::libc::c_int;
    pub fn xdo_wait_for_window_focus(
        xdo: *const xdo_t,
        window: Window,
        want_focus: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_get_pid_window(xdo: *const xdo_t, window: Window) -> ::libc::c_int;
    pub fn xdo_get_focused_window_sane(xdo: *const xdo_t, window_ret: *mut Window)
        -> ::libc::c_int;
    pub fn xdo_activate_window(xdo: *const xdo_t, wid: Window) -> ::libc::c_int;
    pub fn xdo_wait_for_window_active(
        xdo: *const xdo_t,
        window: Window,
        active: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_map_window(xdo: *const xdo_t, wid: Window) -> ::libc::c_int;
    pub fn xdo_unmap_window(xdo: *const xdo_t, wid: Window) -> ::libc::c_int;
    pub fn xdo_minimize_window(xdo: *const xdo_t, wid: Window) -> ::libc::c_int;
    pub fn xdo_reparent_window(
        xdo: *const xdo_t,
        wid_source: Window,
        wid_target: Window,
    ) -> ::libc::c_int;
    pub fn xdo_get_window_location(
        xdo: *const xdo_t,
        wid: Window,
        x_ret: *mut ::libc::c_int,
        y_ret: *mut ::libc::c_int,
        screen_ret: *mut *mut Screen,
    ) -> ::libc::c_int;
    pub fn xdo_get_window_size(
        xdo: *const xdo_t,
        wid: Window,
        width_ret: *mut ::libc::c_uint,
        height_ret: *mut ::libc::c_uint,
    ) -> ::libc::c_int;
    pub fn xdo_get_active_window(xdo: *const xdo_t, window_ret: *mut Window) -> ::libc::c_int;
    pub fn xdo_select_window_with_click(
        xdo: *const xdo_t,
        window_ret: *mut Window,
    ) -> ::libc::c_int;
    pub fn xdo_set_number_of_desktops(
        xdo: *const xdo_t,
        ndesktops: ::libc::c_long,
    ) -> ::libc::c_int;
    pub fn xdo_get_number_of_desktops(
        xdo: *const xdo_t,
        ndesktops: *mut ::libc::c_long,
    ) -> ::libc::c_int;
    pub fn xdo_set_current_desktop(xdo: *const xdo_t, desktop: ::libc::c_long) -> ::libc::c_int;
    pub fn xdo_get_current_desktop(
        xdo: *const xdo_t,
        desktop: *mut ::libc::c_long,
    ) -> ::libc::c_int;
    pub fn xdo_set_desktop_for_window(
        xdo: *const xdo_t,
        wid: Window,
        desktop: ::libc::c_long,
    ) -> ::libc::c_int;
    pub fn xdo_get_desktop_for_window(
        xdo: *const xdo_t,
        wid: Window,
        desktop: *mut ::libc::c_long,
    ) -> ::libc::c_int;
    pub fn xdo_search_windows(
        xdo: *const xdo_t,
        search: *const xdo_search_t,
        windowlist_ret: *mut *mut Window,
        nwindows_ret: *mut ::libc::c_uint,
    ) -> ::libc::c_int;
    pub fn xdo_get_window_property_by_atom(
        xdo: *const xdo_t,
        window: Window,
        atom: Atom,
        nitems: *mut ::libc::c_long,
        _type: *mut Atom,
        size: *mut ::libc::c_int,
    ) -> *mut ::libc::c_uchar;
    pub fn xdo_get_window_property(
        xdo: *const xdo_t,
        window: Window,
        property: *const ::libc::c_char,
        value: *mut *mut ::libc::c_uchar,
        nitems: *mut ::libc::c_long,
        _type: *mut Atom,
        size: *mut ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_get_input_state(xdo: *const xdo_t) -> ::libc::c_uint;
    pub fn xdo_get_symbol_map() -> *mut *const ::libc::c_char;
    pub fn xdo_get_active_modifiers(
        xdo: *const xdo_t,
        keys: *mut *mut charcodemap_t,
        nkeys: *mut ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_clear_active_modifiers(
        xdo: *const xdo_t,
        window: Window,
        active_mods: *mut charcodemap_t,
        active_mods_n: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_set_active_modifiers(
        xdo: *const xdo_t,
        window: Window,
        active_mods: *mut charcodemap_t,
        active_mods_n: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_get_desktop_viewport(
        xdo: *const xdo_t,
        x_ret: *mut ::libc::c_int,
        y_ret: *mut ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_set_desktop_viewport(
        xdo: *const xdo_t,
        x: ::libc::c_int,
        y: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_kill_window(xdo: *const xdo_t, window: Window) -> ::libc::c_int;
    pub fn xdo_find_window_client(
        xdo: *const xdo_t,
        window: Window,
        window_ret: *mut Window,
        direction: ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_get_window_name(
        xdo: *const xdo_t,
        window: Window,
        name_ret: *mut *mut ::libc::c_uchar,
        name_len_ret: *mut ::libc::c_int,
        name_type: *mut ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn xdo_disable_feature(xdo: *mut xdo_t, feature: ::libc::c_int);
    pub fn xdo_enable_feature(xdo: *mut xdo_t, feature: ::libc::c_int);
    pub fn xdo_has_feature(xdo: *mut xdo_t, feature: ::libc::c_int) -> ::libc::c_int;
    pub fn xdo_get_viewport_dimensions(
        xdo: *mut xdo_t,
        width: *mut ::libc::c_uint,
        height: *mut ::libc::c_uint,
        screen: ::libc::c_int,
    ) -> ::libc::c_int;
}
