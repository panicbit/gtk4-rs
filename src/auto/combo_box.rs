// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Accessible;
use AccessibleRole;
use Align;
use Buildable;
use CellEditable;
use CellLayout;
use LayoutManager;
use Overflow;
use ScrollType;
use SensitivityType;
use TreeIter;
use TreeModel;
use Widget;

glib_wrapper! {
    pub struct ComboBox(Object<gtk_sys::GtkComboBox, gtk_sys::GtkComboBoxClass, ComboBoxClass>) @extends Widget, @implements Accessible, Buildable, CellEditable, CellLayout;

    match fn {
        get_type => || gtk_sys::gtk_combo_box_get_type(),
    }
}

impl ComboBox {
    pub fn new() -> ComboBox {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_combo_box_new()).unsafe_cast() }
    }

    pub fn with_entry() -> ComboBox {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_combo_box_new_with_entry()).unsafe_cast() }
    }

    pub fn with_model<P: IsA<TreeModel>>(model: &P) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_combo_box_new_with_model(
                model.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn with_model_and_entry<P: IsA<TreeModel>>(model: &P) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_combo_box_new_with_model_and_entry(
                model.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

impl Default for ComboBox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct ComboBoxBuilder {
    active: Option<i32>,
    active_id: Option<String>,
    button_sensitivity: Option<SensitivityType>,
    child: Option<Widget>,
    entry_text_column: Option<i32>,
    has_entry: Option<bool>,
    has_frame: Option<bool>,
    id_column: Option<i32>,
    model: Option<TreeModel>,
    popup_fixed_width: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
    editing_canceled: Option<bool>,
}

impl ComboBoxBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ComboBox {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref active) = self.active {
            properties.push(("active", active));
        }
        if let Some(ref active_id) = self.active_id {
            properties.push(("active-id", active_id));
        }
        if let Some(ref button_sensitivity) = self.button_sensitivity {
            properties.push(("button-sensitivity", button_sensitivity));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref entry_text_column) = self.entry_text_column {
            properties.push(("entry-text-column", entry_text_column));
        }
        if let Some(ref has_entry) = self.has_entry {
            properties.push(("has-entry", has_entry));
        }
        if let Some(ref has_frame) = self.has_frame {
            properties.push(("has-frame", has_frame));
        }
        if let Some(ref id_column) = self.id_column {
            properties.push(("id-column", id_column));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        if let Some(ref popup_fixed_width) = self.popup_fixed_width {
            properties.push(("popup-fixed-width", popup_fixed_width));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref editing_canceled) = self.editing_canceled {
            properties.push(("editing-canceled", editing_canceled));
        }
        let ret = glib::Object::new(ComboBox::static_type(), &properties)
            .expect("object new")
            .downcast::<ComboBox>()
            .expect("downcast");
        ret
    }

    pub fn active(mut self, active: i32) -> Self {
        self.active = Some(active);
        self
    }

    pub fn active_id(mut self, active_id: &str) -> Self {
        self.active_id = Some(active_id.to_string());
        self
    }

    pub fn button_sensitivity(mut self, button_sensitivity: SensitivityType) -> Self {
        self.button_sensitivity = Some(button_sensitivity);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn entry_text_column(mut self, entry_text_column: i32) -> Self {
        self.entry_text_column = Some(entry_text_column);
        self
    }

    pub fn has_entry(mut self, has_entry: bool) -> Self {
        self.has_entry = Some(has_entry);
        self
    }

    pub fn has_frame(mut self, has_frame: bool) -> Self {
        self.has_frame = Some(has_frame);
        self
    }

    pub fn id_column(mut self, id_column: i32) -> Self {
        self.id_column = Some(id_column);
        self
    }

    pub fn model<P: IsA<TreeModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }

    pub fn popup_fixed_width(mut self, popup_fixed_width: bool) -> Self {
        self.popup_fixed_width = Some(popup_fixed_width);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn editing_canceled(mut self, editing_canceled: bool) -> Self {
        self.editing_canceled = Some(editing_canceled);
        self
    }
}

pub const NONE_COMBO_BOX: Option<&ComboBox> = None;

pub trait ComboBoxExt: 'static {
    fn get_active_id(&self) -> Option<GString>;

    fn get_active_iter(&self) -> Option<TreeIter>;

    fn get_button_sensitivity(&self) -> SensitivityType;

    fn get_child(&self) -> Option<Widget>;

    fn get_entry_text_column(&self) -> i32;

    fn get_has_entry(&self) -> bool;

    fn get_id_column(&self) -> i32;

    fn get_model(&self) -> Option<TreeModel>;

    fn get_popup_fixed_width(&self) -> bool;

    //fn get_row_separator_func(&self) -> Option<Box_<dyn Fn(&TreeModel, &TreeIter) -> bool + 'static>>;

    fn popdown(&self);

    fn popup(&self);

    fn popup_for_device(&self, device: &gdk::Device);

    fn set_active_id(&self, active_id: Option<&str>) -> bool;

    fn set_active_iter(&self, iter: Option<&TreeIter>);

    fn set_button_sensitivity(&self, sensitivity: SensitivityType);

    fn set_child<P: IsA<Widget>>(&self, child: Option<&P>);

    fn set_entry_text_column(&self, text_column: i32);

    fn set_id_column(&self, id_column: i32);

    fn set_model<P: IsA<TreeModel>>(&self, model: Option<&P>);

    fn set_popup_fixed_width(&self, fixed: bool);

    fn set_row_separator_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(&self, func: P);

    fn get_property_has_frame(&self) -> bool;

    fn set_property_has_frame(&self, has_frame: bool);

    fn get_property_popup_shown(&self) -> bool;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_format_entry_text<F: Fn(&Self, &str) -> String + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_move_active<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_active(&self, scroll_type: ScrollType);

    fn connect_popdown<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popdown(&self) -> bool;

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popup(&self);

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_button_sensitivity_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_entry_text_column_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_has_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_id_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_fixed_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_popup_shown_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ComboBox>> ComboBoxExt for O {
    fn get_active_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_combo_box_get_active_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_active_iter(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(gtk_sys::gtk_combo_box_get_active_iter(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    fn get_button_sensitivity(&self) -> SensitivityType {
        unsafe {
            from_glib(gtk_sys::gtk_combo_box_get_button_sensitivity(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_combo_box_get_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_entry_text_column(&self) -> i32 {
        unsafe { gtk_sys::gtk_combo_box_get_entry_text_column(self.as_ref().to_glib_none().0) }
    }

    fn get_has_entry(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_combo_box_get_has_entry(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_id_column(&self) -> i32 {
        unsafe { gtk_sys::gtk_combo_box_get_id_column(self.as_ref().to_glib_none().0) }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_combo_box_get_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_popup_fixed_width(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_combo_box_get_popup_fixed_width(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_row_separator_func(&self) -> Option<Box_<dyn Fn(&TreeModel, &TreeIter) -> bool + 'static>> {
    //    unsafe { TODO: call gtk_sys:gtk_combo_box_get_row_separator_func() }
    //}

    fn popdown(&self) {
        unsafe {
            gtk_sys::gtk_combo_box_popdown(self.as_ref().to_glib_none().0);
        }
    }

    fn popup(&self) {
        unsafe {
            gtk_sys::gtk_combo_box_popup(self.as_ref().to_glib_none().0);
        }
    }

    fn popup_for_device(&self, device: &gdk::Device) {
        unsafe {
            gtk_sys::gtk_combo_box_popup_for_device(
                self.as_ref().to_glib_none().0,
                device.to_glib_none().0,
            );
        }
    }

    fn set_active_id(&self, active_id: Option<&str>) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_combo_box_set_active_id(
                self.as_ref().to_glib_none().0,
                active_id.to_glib_none().0,
            ))
        }
    }

    fn set_active_iter(&self, iter: Option<&TreeIter>) {
        unsafe {
            gtk_sys::gtk_combo_box_set_active_iter(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            );
        }
    }

    fn set_button_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            gtk_sys::gtk_combo_box_set_button_sensitivity(
                self.as_ref().to_glib_none().0,
                sensitivity.to_glib(),
            );
        }
    }

    fn set_child<P: IsA<Widget>>(&self, child: Option<&P>) {
        unsafe {
            gtk_sys::gtk_combo_box_set_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_entry_text_column(&self, text_column: i32) {
        unsafe {
            gtk_sys::gtk_combo_box_set_entry_text_column(
                self.as_ref().to_glib_none().0,
                text_column,
            );
        }
    }

    fn set_id_column(&self, id_column: i32) {
        unsafe {
            gtk_sys::gtk_combo_box_set_id_column(self.as_ref().to_glib_none().0, id_column);
        }
    }

    fn set_model<P: IsA<TreeModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_combo_box_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_popup_fixed_width(&self, fixed: bool) {
        unsafe {
            gtk_sys::gtk_combo_box_set_popup_fixed_width(
                self.as_ref().to_glib_none().0,
                fixed.to_glib(),
            );
        }
    }

    fn set_row_separator_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(&self, func: P) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(
            model: *mut gtk_sys::GtkTreeModel,
            iter: *mut gtk_sys::GtkTreeIter,
            data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let model = from_glib_borrow(model);
            let iter = from_glib_borrow(iter);
            let callback: &P = &*(data as *mut _);
            let res = (*callback)(&model, &iter);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(
            data: glib_sys::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            gtk_sys::gtk_combo_box_set_row_separator_func(
                self.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn get_property_has_frame(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"has-frame\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `has-frame` getter")
                .unwrap()
        }
    }

    fn set_property_has_frame(&self, has_frame: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"has-frame\0".as_ptr() as *const _,
                Value::from(&has_frame).to_glib_none().0,
            );
        }
    }

    fn get_property_popup_shown(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"popup-shown\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `popup-shown` getter")
                .unwrap()
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_format_entry_text<F: Fn(&Self, &str) -> String + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn format_entry_text_trampoline<P, F: Fn(&P, &str) -> String + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            path: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) -> *mut libc::c_char
        where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ComboBox::from_glib_borrow(this).unsafe_cast_ref(),
                &GString::from_glib_borrow(path),
            )
            .to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"format-entry-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    format_entry_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_move_active<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn move_active_trampoline<P, F: Fn(&P, ScrollType) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            scroll_type: gtk_sys::GtkScrollType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ComboBox::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(scroll_type),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_active(&self, scroll_type: ScrollType) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("move-active", &[&scroll_type])
                .unwrap()
        };
    }

    fn connect_popdown<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn popdown_trampoline<P, F: Fn(&P) -> bool + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref()).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"popdown\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    popdown_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_popdown(&self) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("popdown", &[])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_popdown`")
            .unwrap()
    }

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn popup_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    popup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_popup(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("popup", &[])
                .unwrap()
        };
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_active_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_button_sensitivity_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_button_sensitivity_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::button-sensitivity\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_button_sensitivity_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_entry_text_column_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_entry_text_column_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::entry-text-column\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_entry_text_column_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_has_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_frame_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-frame\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_frame_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_id_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_column_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id-column\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_column_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_popup_fixed_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_fixed_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::popup-fixed-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_popup_fixed_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_popup_shown_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_shown_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkComboBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ComboBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::popup-shown\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_popup_shown_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ComboBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComboBox")
    }
}
