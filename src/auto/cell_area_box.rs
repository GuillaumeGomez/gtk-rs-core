// This file was generated by gir (b798f4f) from gir-files (11e0e6d)
// DO NOT EDIT

use CellArea;
use CellLayout;
use CellRenderer;
use Orientable;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct CellAreaBox(Object<ffi::GtkCellAreaBox>): CellArea, CellLayout, Orientable;

    match fn {
        get_type => || ffi::gtk_cell_area_box_get_type(),
    }
}

impl CellAreaBox {
    pub fn new() -> CellAreaBox {
        assert_initialized_main_thread!();
        unsafe {
            CellArea::from_glib_none(ffi::gtk_cell_area_box_new()).downcast_unchecked()
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_cell_area_box_get_spacing(self.to_glib_none().0)
        }
    }

    pub fn pack_end<T: IsA<CellRenderer>>(&self, renderer: &T, expand: bool, align: bool, fixed: bool) {
        unsafe {
            ffi::gtk_cell_area_box_pack_end(self.to_glib_none().0, renderer.to_glib_none().0, expand.to_glib(), align.to_glib(), fixed.to_glib());
        }
    }

    pub fn pack_start<T: IsA<CellRenderer>>(&self, renderer: &T, expand: bool, align: bool, fixed: bool) {
        unsafe {
            ffi::gtk_cell_area_box_pack_start(self.to_glib_none().0, renderer.to_glib_none().0, expand.to_glib(), align.to_glib(), fixed.to_glib());
        }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_cell_area_box_set_spacing(self.to_glib_none().0, spacing);
        }
    }
}
