// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;

glib_wrapper! {
    pub struct AccelGroup(Object<gtk_sys::GtkAccelGroup, gtk_sys::GtkAccelGroupClass, AccelGroupClass>);

    match fn {
        get_type => || gtk_sys::gtk_accel_group_get_type(),
    }
}

impl AccelGroup {
    pub fn new() -> AccelGroup {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_accel_group_new()) }
    }

    pub fn from_accel_closure(closure: &glib::Closure) -> Option<AccelGroup> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gtk_sys::gtk_accel_group_from_accel_closure(
                closure.to_glib_none().0,
            ))
        }
    }
}

impl Default for AccelGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ACCEL_GROUP: Option<&AccelGroup> = None;

pub trait AccelGroupExt: 'static {
    fn activate<P: IsA<glib::Object>>(
        &self,
        accel_quark: glib::Quark,
        acceleratable: &P,
        accel_key: u32,
        accel_mods: gdk::ModifierType,
    ) -> bool;

    fn disconnect(&self, closure: Option<&glib::Closure>) -> bool;

    fn disconnect_key(&self, accel_key: u32, accel_mods: gdk::ModifierType) -> bool;

    //fn find(&self, find_func: /*Unimplemented*/FnMut(/*Ignored*/AccelKey, &glib::Closure) -> bool, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Ignored*/Option<AccelKey>;

    fn get_is_locked(&self) -> bool;

    fn get_modifier_mask(&self) -> gdk::ModifierType;

    fn lock(&self);

    fn unlock(&self);

    fn connect_accel_activate<
        F: Fn(&Self, &glib::Object, u32, gdk::ModifierType) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_accel_changed<F: Fn(&Self, u32, gdk::ModifierType, &glib::Closure) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_modifier_mask_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AccelGroup>> AccelGroupExt for O {
    fn activate<P: IsA<glib::Object>>(
        &self,
        accel_quark: glib::Quark,
        acceleratable: &P,
        accel_key: u32,
        accel_mods: gdk::ModifierType,
    ) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_accel_group_activate(
                self.as_ref().to_glib_none().0,
                accel_quark.to_glib(),
                acceleratable.as_ref().to_glib_none().0,
                accel_key,
                accel_mods.to_glib(),
            ))
        }
    }

    fn disconnect(&self, closure: Option<&glib::Closure>) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_accel_group_disconnect(
                self.as_ref().to_glib_none().0,
                closure.to_glib_none().0,
            ))
        }
    }

    fn disconnect_key(&self, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_accel_group_disconnect_key(
                self.as_ref().to_glib_none().0,
                accel_key,
                accel_mods.to_glib(),
            ))
        }
    }

    //fn find(&self, find_func: /*Unimplemented*/FnMut(/*Ignored*/AccelKey, &glib::Closure) -> bool, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Ignored*/Option<AccelKey> {
    //    unsafe { TODO: call gtk_sys:gtk_accel_group_find() }
    //}

    fn get_is_locked(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_accel_group_get_is_locked(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_modifier_mask(&self) -> gdk::ModifierType {
        unsafe {
            from_glib(gtk_sys::gtk_accel_group_get_modifier_mask(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn lock(&self) {
        unsafe {
            gtk_sys::gtk_accel_group_lock(self.as_ref().to_glib_none().0);
        }
    }

    fn unlock(&self) {
        unsafe {
            gtk_sys::gtk_accel_group_unlock(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_accel_activate<
        F: Fn(&Self, &glib::Object, u32, gdk::ModifierType) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accel_activate_trampoline<
            P,
            F: Fn(&P, &glib::Object, u32, gdk::ModifierType) -> bool + 'static,
        >(
            this: *mut gtk_sys::GtkAccelGroup,
            acceleratable: *mut gobject_sys::GObject,
            keyval: libc::c_uint,
            modifier: gdk_sys::GdkModifierType,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<AccelGroup>,
        {
            let f: &F = &*(f as *const F);
            f(
                &AccelGroup::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(acceleratable),
                keyval,
                from_glib(modifier),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accel-activate\0".as_ptr() as *const _,
                Some(*(&accel_activate_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_accel_changed<F: Fn(&Self, u32, gdk::ModifierType, &glib::Closure) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accel_changed_trampoline<
            P,
            F: Fn(&P, u32, gdk::ModifierType, &glib::Closure) + 'static,
        >(
            this: *mut gtk_sys::GtkAccelGroup,
            keyval: libc::c_uint,
            modifier: gdk_sys::GdkModifierType,
            accel_closure: *mut gobject_sys::GClosure,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AccelGroup>,
        {
            let f: &F = &*(f as *const F);
            f(
                &AccelGroup::from_glib_borrow(this).unsafe_cast_ref(),
                keyval,
                from_glib(modifier),
                &from_glib_borrow(accel_closure),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accel-changed\0".as_ptr() as *const _,
                Some(*(&accel_changed_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_locked_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAccelGroup,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AccelGroup>,
        {
            let f: &F = &*(f as *const F);
            f(&AccelGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-locked\0".as_ptr() as *const _,
                Some(*(&notify_is_locked_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_modifier_mask_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_modifier_mask_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAccelGroup,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AccelGroup>,
        {
            let f: &F = &*(f as *const F);
            f(&AccelGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modifier-mask\0".as_ptr() as *const _,
                Some(*(&notify_modifier_mask_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for AccelGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AccelGroup")
    }
}
