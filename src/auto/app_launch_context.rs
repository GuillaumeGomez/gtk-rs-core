// This file was generated by gir (8343e00) from gir-files (71d73f0)
// DO NOT EDIT

use Display;
use Screen;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct AppLaunchContext(Object<ffi::GdkAppLaunchContext>);

    match fn {
        get_type => || ffi::gdk_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    pub fn new() -> AppLaunchContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_app_launch_context_new())
        }
    }
}

pub trait AppLaunchContextExt {
    fn set_desktop(&self, desktop: i32);

    fn set_display(&self, display: &Display);

    //fn set_icon<'a, P: IsA</*Ignored*/gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, icon: Q);

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P);

    fn set_screen(&self, screen: &Screen);

    fn set_timestamp(&self, timestamp: u32);

    fn get_property_display(&self) -> Option<Display>;
}

impl<O: IsA<AppLaunchContext> + IsA<glib::object::Object>> AppLaunchContextExt for O {
    fn set_desktop(&self, desktop: i32) {
        unsafe {
            ffi::gdk_app_launch_context_set_desktop(self.to_glib_none().0, desktop);
        }
    }

    fn set_display(&self, display: &Display) {
        unsafe {
            ffi::gdk_app_launch_context_set_display(self.to_glib_none().0, display.to_glib_none().0);
        }
    }

    //fn set_icon<'a, P: IsA</*Ignored*/gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, icon: Q) {
    //    unsafe { TODO: call ffi::gdk_app_launch_context_set_icon() }
    //}

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P) {
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            ffi::gdk_app_launch_context_set_icon_name(self.to_glib_none().0, icon_name.0);
        }
    }

    fn set_screen(&self, screen: &Screen) {
        unsafe {
            ffi::gdk_app_launch_context_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    fn set_timestamp(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_app_launch_context_set_timestamp(self.to_glib_none().0, timestamp);
        }
    }

    fn get_property_display(&self) -> Option<Display> {
        let mut value = Value::from(None::<&Display>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "display".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }
}
