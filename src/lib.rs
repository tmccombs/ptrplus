use std::cell::{Cell, RefCell};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

pub trait AsPtr {
    type Raw;
    fn as_ptr(&self) -> *const Self::Raw;
}


impl<T> AsPtr for [T] {
    type Raw = T;
    fn as_ptr(&self) -> *const T {
        <[T]>::as_ptr(self)
    }
}

impl AsPtr for CStr {
    type Raw = c_char;
    fn as_ptr(&self) -> *const c_char {
        CStr::as_ptr(self)
    }
}

impl<'a, T> AsPtr for &'a T where T: Sized {
    type Raw = T;
    fn as_ptr(&self) -> *const T {
        *self as *const T
    }
}

impl<T> AsPtr for *const T {
    type Raw = T;
    fn as_ptr(&self) -> *const T {
        *self
    }
}

impl<T> AsPtr for Cell<T> where T: Copy {
    type Raw = T;
    fn as_ptr(&self) -> *const T {
        Cell::as_ptr(self)
    }
}

impl<T> AsPtr for RefCell<T> {
    type Raw = T;
    fn as_ptr(&self) -> *const T {
        RefCell::as_ptr(self)
    }
}


impl<T> AsPtr for Option<T> where T: AsPtr {
    type Raw = T::Raw;
    fn as_ptr(&self) -> *const T::Raw {
        match self {
            &Some(ref v) => v.as_ptr(),
            &None => ptr::null()
        }
    }
}

pub trait IntoRaw {
    type Raw;
    fn into_raw(self) -> *mut Self::Raw;
}

impl<T> IntoRaw for Box<T> {
    type Raw = T;
    fn into_raw(self) -> *mut T {
        Box::into_raw(self)
    }
}

impl IntoRaw for CString {
    type Raw = c_char;
    fn into_raw(self) -> *mut c_char {
        CString::into_raw(self)
    }
}

impl<T> IntoRaw for *mut T {
    type Raw = T;
    fn into_raw(self) -> *mut T {
        self
    }
}

impl<T> IntoRaw for Option<T> where T: IntoRaw {
    type Raw = T::Raw;
    fn into_raw(self) -> *mut T::Raw {
        match self {
            Some(v) => v.into_raw(),
            None => ptr::null_mut()
        }
    }
}
