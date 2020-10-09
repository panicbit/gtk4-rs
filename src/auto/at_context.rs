// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use Accessible;
use AccessibleRole;

glib_wrapper! {
    pub struct ATContext(Object<gtk_sys::GtkATContext, gtk_sys::GtkATContextClass, ATContextClass>);

    match fn {
        get_type => || gtk_sys::gtk_at_context_get_type(),
    }
}

impl ATContext {
    pub fn create<P: IsA<Accessible>>(
        accessible_role: AccessibleRole,
        accessible: &P,
    ) -> Option<ATContext> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_sys::gtk_at_context_create(
                accessible_role.to_glib(),
                accessible.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn get_accessible(&self) -> Option<Accessible> {
        unsafe {
            from_glib_none(gtk_sys::gtk_at_context_get_accessible(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_accessible_role(&self) -> AccessibleRole {
        unsafe {
            from_glib(gtk_sys::gtk_at_context_get_accessible_role(
                self.to_glib_none().0,
            ))
        }
    }

    //pub fn connect_state_change<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented states: *.Pointer
    //    Unimplemented properties: *.Pointer
    //    Unimplemented relations: *.Pointer
    //}
}

impl fmt::Display for ATContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ATContext")
    }
}
