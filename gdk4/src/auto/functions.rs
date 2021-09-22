// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Texture;
use glib::object::IsA;
use glib::translate::*;

#[doc(alias = "gdk_intern_mime_type")]
pub fn intern_mime_type(string: &str) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gdk_intern_mime_type(string.to_glib_none().0)) }
}

#[doc(alias = "gdk_pixbuf_get_from_surface")]
pub fn pixbuf_get_from_surface(
    surface: &cairo::Surface,
    src_x: i32,
    src_y: i32,
    width: i32,
    height: i32,
) -> Option<gdk_pixbuf::Pixbuf> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gdk_pixbuf_get_from_surface(
            mut_override(surface.to_glib_none().0),
            src_x,
            src_y,
            width,
            height,
        ))
    }
}

#[doc(alias = "gdk_pixbuf_get_from_texture")]
pub fn pixbuf_get_from_texture(texture: &impl IsA<Texture>) -> Option<gdk_pixbuf::Pixbuf> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gdk_pixbuf_get_from_texture(
            texture.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "gdk_set_allowed_backends")]
pub fn set_allowed_backends(backends: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_allowed_backends(backends.to_glib_none().0);
    }
}

#[doc(alias = "gdk_unicode_to_keyval")]
pub fn unicode_to_keyval(wc: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_unicode_to_keyval(wc) }
}
