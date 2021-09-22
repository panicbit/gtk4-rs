// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Widget;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkWidgetPaintable")]
    pub struct WidgetPaintable(Object<ffi::GtkWidgetPaintable, ffi::GtkWidgetPaintableClass>) @implements gdk::Paintable;

    match fn {
        type_ => || ffi::gtk_widget_paintable_get_type(),
    }
}

impl WidgetPaintable {
    #[doc(alias = "gtk_widget_paintable_new")]
    pub fn new(widget: Option<&impl IsA<Widget>>) -> WidgetPaintable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_widget_paintable_new(
                widget.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_widget_paintable_get_widget")]
    #[doc(alias = "get_widget")]
    pub fn widget(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_widget_paintable_get_widget(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_widget_paintable_set_widget")]
    pub fn set_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_widget_paintable_set_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "widget")]
    pub fn connect_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_widget_trampoline<F: Fn(&WidgetPaintable) + 'static>(
            this: *mut ffi::GtkWidgetPaintable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_widget_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for WidgetPaintable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WidgetPaintable")
    }
}
