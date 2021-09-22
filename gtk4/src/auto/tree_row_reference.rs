// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::TreeModel;
use crate::TreePath;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TreeRowReference(Boxed<ffi::GtkTreeRowReference>);

    match fn {
        copy => |ptr| ffi::gtk_tree_row_reference_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_tree_row_reference_free(ptr),
        type_ => || ffi::gtk_tree_row_reference_get_type(),
    }
}

impl TreeRowReference {
    #[doc(alias = "gtk_tree_row_reference_new")]
    pub fn new(model: &impl IsA<TreeModel>, path: &TreePath) -> Option<TreeRowReference> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_tree_row_reference_new(
                model.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
            ))
        }
    }

    #[doc(alias = "gtk_tree_row_reference_new_proxy")]
    pub fn new_proxy(
        proxy: &impl IsA<glib::Object>,
        model: &impl IsA<TreeModel>,
        path: &TreePath,
    ) -> Option<TreeRowReference> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_tree_row_reference_new_proxy(
                proxy.as_ref().to_glib_none().0,
                model.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
            ))
        }
    }

    #[doc(alias = "gtk_tree_row_reference_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> TreeModel {
        unsafe {
            from_glib_none(ffi::gtk_tree_row_reference_get_model(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gtk_tree_row_reference_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&self) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_row_reference_get_path(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gtk_tree_row_reference_valid")]
    pub fn valid(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_row_reference_valid(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gtk_tree_row_reference_deleted")]
    pub fn deleted(proxy: &impl IsA<glib::Object>, path: &TreePath) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_tree_row_reference_deleted(
                proxy.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gtk_tree_row_reference_inserted")]
    pub fn inserted(proxy: &impl IsA<glib::Object>, path: &TreePath) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_tree_row_reference_inserted(
                proxy.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
            );
        }
    }
}
