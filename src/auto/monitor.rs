// This file was generated by gir (8343e00) from gir-files (71d73f0)
// DO NOT EDIT

use Display;
use Rectangle;
#[cfg(feature = "v3_22")]
use SubpixelLayout;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Monitor(Object<ffi::GdkMonitor>);

    match fn {
        get_type => || ffi::gdk_monitor_get_type(),
    }
}

pub trait MonitorExt {
    #[cfg(feature = "v3_22")]
    fn get_display(&self) -> Option<Display>;

    #[cfg(feature = "v3_22")]
    fn get_geometry(&self) -> Rectangle;

    #[cfg(feature = "v3_22")]
    fn get_height_mm(&self) -> i32;

    fn get_manufacturer(&self) -> Option<String>;

    fn get_model(&self) -> Option<String>;

    #[cfg(feature = "v3_22")]
    fn get_refresh_rate(&self) -> i32;

    #[cfg(feature = "v3_22")]
    fn get_scale_factor(&self) -> i32;

    #[cfg(feature = "v3_22")]
    fn get_subpixel_layout(&self) -> SubpixelLayout;

    #[cfg(feature = "v3_22")]
    fn get_width_mm(&self) -> i32;

    #[cfg(feature = "v3_22")]
    fn get_workarea(&self) -> Rectangle;

    #[cfg(feature = "v3_22")]
    fn is_primary(&self) -> bool;

    fn get_property_display(&self) -> Option<Display>;

    fn get_property_geometry(&self) -> Option<Rectangle>;

    fn get_property_height_mm(&self) -> i32;

    fn get_property_refresh_rate(&self) -> i32;

    fn get_property_scale_factor(&self) -> i32;

    fn get_property_width_mm(&self) -> i32;

    fn get_property_workarea(&self) -> Option<Rectangle>;

    fn connect_invalidate<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Monitor> + IsA<glib::object::Object>> MonitorExt for O {
    #[cfg(feature = "v3_22")]
    fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_display(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_geometry(&self) -> Rectangle {
        unsafe {
            let mut geometry = Rectangle::uninitialized();
            ffi::gdk_monitor_get_geometry(self.to_glib_none().0, geometry.to_glib_none_mut().0);
            geometry
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_height_mm(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_height_mm(self.to_glib_none().0)
        }
    }

    fn get_manufacturer(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_manufacturer(self.to_glib_none().0))
        }
    }

    fn get_model(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_model(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_refresh_rate(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_refresh_rate(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_scale_factor(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_scale_factor(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_subpixel_layout(&self) -> SubpixelLayout {
        unsafe {
            from_glib(ffi::gdk_monitor_get_subpixel_layout(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_width_mm(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_width_mm(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_workarea(&self) -> Rectangle {
        unsafe {
            let mut workarea = Rectangle::uninitialized();
            ffi::gdk_monitor_get_workarea(self.to_glib_none().0, workarea.to_glib_none_mut().0);
            workarea
        }
    }

    #[cfg(feature = "v3_22")]
    fn is_primary(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_monitor_is_primary(self.to_glib_none().0))
        }
    }

    fn get_property_display(&self) -> Option<Display> {
        let mut value = Value::from(None::<&Display>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "display".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_geometry(&self) -> Option<Rectangle> {
        let mut value = Value::from(None::<&Rectangle>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "geometry".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_height_mm(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "height-mm".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_refresh_rate(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "refresh-rate".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_scale_factor(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scale-factor".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_width_mm(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "width-mm".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_workarea(&self) -> Option<Rectangle> {
        let mut value = Value::from(None::<&Rectangle>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "workarea".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn connect_invalidate<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "invalidate",
                transmute(invalidate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn invalidate_trampoline<P>(this: *mut ffi::GdkMonitor, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Monitor::from_glib_none(this).downcast_unchecked())
}
