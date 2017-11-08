// This file was generated by gir (0409d73) from gir-files (469db10)
// DO NOT EDIT

use Adjustment;
use Bin;
use Container;
use Scrollable;
use ShadowType;
use Widget;
use ffi;
use gdk;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Viewport(Object<ffi::GtkViewport, ffi::GtkViewportClass>): Bin, Container, Widget, Scrollable;

    match fn {
        get_type => || ffi::gtk_viewport_get_type(),
    }
}

impl Viewport {
    pub fn new<'a, 'b, P: Into<Option<&'a Adjustment>>, Q: Into<Option<&'b Adjustment>>>(hadjustment: P, vadjustment: Q) -> Viewport {
        assert_initialized_main_thread!();
        let hadjustment = hadjustment.into();
        let hadjustment = hadjustment.to_glib_none();
        let vadjustment = vadjustment.into();
        let vadjustment = vadjustment.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_viewport_new(hadjustment.0, vadjustment.0)).downcast_unchecked()
        }
    }
}

pub trait ViewportExt {
    fn get_bin_window(&self) -> Option<gdk::Window>;

    fn get_shadow_type(&self) -> ShadowType;

    fn get_view_window(&self) -> Option<gdk::Window>;

    fn set_shadow_type(&self, type_: ShadowType);

    fn connect_property_shadow_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Viewport> + IsA<glib::object::Object>> ViewportExt for O {
    fn get_bin_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_viewport_get_bin_window(self.to_glib_none().0))
        }
    }

    fn get_shadow_type(&self) -> ShadowType {
        unsafe {
            from_glib(ffi::gtk_viewport_get_shadow_type(self.to_glib_none().0))
        }
    }

    fn get_view_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_viewport_get_view_window(self.to_glib_none().0))
        }
    }

    fn set_shadow_type(&self, type_: ShadowType) {
        unsafe {
            ffi::gtk_viewport_set_shadow_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    fn connect_property_shadow_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::shadow-type",
                transmute(notify_shadow_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_shadow_type_trampoline<P>(this: *mut ffi::GtkViewport, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Viewport> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Viewport::from_glib_borrow(this).downcast_unchecked())
}
