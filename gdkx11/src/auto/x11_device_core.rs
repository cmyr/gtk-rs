// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct X11DeviceCore(Object<ffi::GdkX11DeviceCore, ffi::GdkX11DeviceCoreClass>) @extends gdk::Device;

    match fn {
        get_type => || ffi::gdk_x11_device_core_get_type(),
    }
}

impl X11DeviceCore {}

impl fmt::Display for X11DeviceCore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11DeviceCore")
    }
}