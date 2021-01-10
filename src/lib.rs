#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "alloc")]
extern crate alloc;

use core::cell::{Cell, RefCell};
#[cfg(feature = "alloc")]
use core::ops::Deref;
use core::ptr::{self, NonNull};
#[cfg(feature = "std")]
use std::ffi::{CStr, CString};
#[cfg(feature = "std")]
use std::os::raw::c_char;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::rc::Rc;
#[cfg(feature = "alloc")]
use alloc::sync::Arc;

macro_rules! asptr_wrapper {
    ($name:ident) => {
        impl<T> AsPtr for $name<T> {
            type Raw = T;
            #[inline]
            fn as_ptr(&self) -> *const T {
                $name::as_ptr(self)
            }
        }
    };
}

#[cfg(feature = "alloc")]
macro_rules! owned_ptr_wrapper {
    ($name:ident) => {
        impl<T> IntoRaw for $name<T> {
            type Raw = T;
            fn into_raw(self) -> *mut T {
                $name::into_raw(self) as *mut T
            }
        }
        impl<T> FromRaw<T> for $name<T> {
            unsafe fn from_raw(raw: *mut T) -> $name<T> {
                $name::from_raw(raw)
            }
        }
    };
}

/// Trait for types that implement `as_ptr`.
///
/// This is implemented by types which can be converted
/// to a pointer from a borrowed reference.
///
/// # Example
/// ```
/// use ptrplus::AsPtr;
///
/// let x: &u32 = &5;
/// let y: *const u32 = x.as_ptr();
/// unsafe {
///     assert_eq!(*y, 5);
/// }
/// ```
///
/// ```
/// use ptrplus::AsPtr;
///
/// let x = 5;
/// let o1: Option<&u32> = None;
/// let o2: Option<&u32> = Some(&x);
///
/// assert!(o1.as_ptr().is_null());
/// assert!(!o2.as_ptr().is_null());
/// unsafe {
///     assert_eq!(*o2.as_ptr(), 5);
/// }
/// ```
pub trait AsPtr {
    /// The type pointed to
    ///
    /// `as_ptr` will return a pointer to this type
    type Raw;

    /// Returns a raw pointer to the contained content
    ///
    /// The caller must ensure `self` outlives the pointer
    /// that is returned, or else it will end up pointing
    /// to garbage.
    ///
    /// Mutating `self` may also invalidate this pointer,
    /// depending on the implementation.
    fn as_ptr(&self) -> *const Self::Raw;
}

impl<T> AsPtr for [T] {
    type Raw = T;
    #[inline]
    fn as_ptr(&self) -> *const T {
        <[T]>::as_ptr(self)
    }
}

impl<'a, T> AsPtr for &'a T
where
    T: Sized,
{
    type Raw = T;
    #[inline]
    fn as_ptr(&self) -> *const T {
        *self as *const T
    }
}

impl<T> AsPtr for NonNull<T> {
    type Raw = T;
    #[inline]
    fn as_ptr(&self) -> *const T {
        NonNull::as_ptr(*self)
    }
}

impl<T> AsPtr for *const T {
    type Raw = T;
    #[inline]
    fn as_ptr(&self) -> *const T {
        *self
    }
}

#[cfg(feature = "std")]
impl AsPtr for CStr {
    type Raw = c_char;
    #[inline]
    fn as_ptr(&self) -> *const c_char {
        CStr::as_ptr(self)
    }
}

#[cfg(feature = "std")]
impl AsPtr for CString {
    type Raw = c_char;
    #[inline]
    fn as_ptr(&self) -> *const c_char {
        CStr::as_ptr(self)
    }
}

#[cfg(feature = "alloc")]
impl<T> AsPtr for Box<T> {
    type Raw = T;
    #[inline]
    fn as_ptr(&self) -> *const T {
        self.deref().as_ptr()
    }
}

impl<T> AsPtr for Option<T>
where
    T: AsPtr,
{
    type Raw = T::Raw;
    #[inline]
    fn as_ptr(&self) -> *const T::Raw {
        match self {
            Some(ref v) => v.as_ptr(),
            None => ptr::null(),
        }
    }
}

asptr_wrapper!(Cell);
asptr_wrapper!(RefCell);
#[cfg(feature = "alloc")]
asptr_wrapper!(Rc);
#[cfg(feature = "alloc")]
asptr_wrapper!(Arc);

/// Trait for types that implement `into_raw`
///
/// This is implemented by types that can be converted
/// into a pointer by consuming ownership of the object
///
/// # Example
/// ```
/// use ptrplus::IntoRaw;
///
/// let x: Box<u32> = Box::new(5);
/// let y: *mut u32 = IntoRaw::into_raw(x);
/// unsafe {
///   assert_eq!(*y, 5);
///   *y = 6;
///   Box::from_raw(y);
/// }
///
/// ```
///
/// ```
/// use ptrplus::{FromRaw, IntoRaw};
///
/// let o1: Option<Box<u32>> = None;
/// let o2: Option<Box<u32>> = Some(Box::new(5));
///
/// let p1: *mut u32 = o1.into_raw();
/// let p2: *mut u32 = o2.into_raw();
///
/// assert!(p1.is_null());
/// assert!(!p2.is_null());
/// unsafe {
///     assert_eq!(*p2, 5);
///     let o1: Option<Box<u32>> = Option::from_raw(p1);
///     let o2: Option<Box<u32>> = Option::from_raw(p2);
///     assert!(o1.is_none());
///     assert!(!o2.is_none());
/// }
/// ```
pub trait IntoRaw {
    /// The type pointed to
    ///
    /// `into_raw` returns a mutable pointer to this type
    type Raw;

    /// Consumes `self` returning the wrapped raw pointer.
    ///
    /// After calling this method, the caller is responsable
    /// for making sure any resources attached to this pointer
    /// (such as memory) are cleaned up. The proper way to do this
    /// is to convert the pointer back to `Self`.
    ///
    /// See `FromRaw`
    fn into_raw(self) -> *mut Self::Raw;
}

#[cfg(feature = "std")]
impl IntoRaw for CString {
    type Raw = c_char;
    #[inline]
    fn into_raw(self) -> *mut c_char {
        CString::into_raw(self)
    }
}

impl<T> IntoRaw for *mut T {
    type Raw = T;
    #[inline]
    fn into_raw(self) -> *mut T {
        self
    }
}

impl<T> IntoRaw for NonNull<T> {
    type Raw = T;
    #[inline]
    fn into_raw(self) -> *mut T {
        self.as_ptr()
    }
}

impl<T> IntoRaw for Option<T>
where
    T: IntoRaw,
{
    type Raw = T::Raw;
    #[inline]
    fn into_raw(self) -> *mut T::Raw {
        match self {
            Some(v) => v.into_raw(),
            None => ptr::null_mut(),
        }
    }
}

/// Trait for types that can be created from a raw pointer
///
/// # Examples
/// ```
/// use ptrplus::{FromRaw, IntoRaw};
///
/// let x: Box<u32> = Box::new(5);
/// let y = x.into_raw();
/// let z: Box<u32> = unsafe { FromRaw::from_raw(y) };
/// assert_eq!(*z, 5);
///
/// ```
///
pub trait FromRaw<T> {
    /// Create `Self` from a raw pointer
    ///
    /// After calling this method the raw pointer
    /// is owned by the resulting object. This
    /// means that the resulting object should
    /// clean up any resources associated with
    /// the pointer (such as memory).
    ///
    /// # Safety
    ///
    /// `raw` must be a pointer that is compatible with
    /// the resulting type. For example, if `Self` is
    /// `Box<T>`, then `raw` must be a pointer to memory allocated
    /// as a Box. The exact requirements depend on the implementation.
    ///
    /// Generally, the `raw` pointer must be the result of a previous
    /// call to `into_raw` on the corresponding type. This the case for
    /// types such as `Box`, `Rc`, and `Arc`.
    ///
    /// Additionally, this function takes ownership of the pointer. If
    /// `raw` or an alias thereof is used after calling this function
    /// it can potentially result in double-free, data races, or other
    /// undefined behavior.
    unsafe fn from_raw(raw: *mut T) -> Self;
}

#[cfg(feature = "std")]
impl FromRaw<c_char> for CString {
    #[inline]
    unsafe fn from_raw(raw: *mut c_char) -> CString {
        CString::from_raw(raw)
    }
}

/// This implementation is always safe
impl<T> FromRaw<T> for *mut T {
    #[inline]
    unsafe fn from_raw(raw: *mut T) -> *mut T {
        raw
    }
}

/// This implementation is always safe
impl<T> FromRaw<T> for *const T {
    #[inline]
    unsafe fn from_raw(raw: *mut T) -> *const T {
        raw
    }
}

/// ## Safety
/// The input pointer must be non-null.
///
/// `Option<NonNull<T>>::from_raw` can be used if the pointer may be null.
impl<T> FromRaw<T> for NonNull<T> {
    #[inline]
    unsafe fn from_raw(raw: *mut T) -> NonNull<T> {
        NonNull::new_unchecked(raw)
    }
}

/// ## Safety
/// The input pointer must either be null (resulting in `None`), or be safe
/// to convert into the inner pointer type.
impl<T, U> FromRaw<U> for Option<T>
where
    T: FromRaw<U>,
{
    unsafe fn from_raw(raw: *mut U) -> Option<T> {
        if raw.is_null() {
            None
        } else {
            Some(T::from_raw(raw))
        }
    }
}

#[cfg(feature = "alloc")]
owned_ptr_wrapper!(Box);
#[cfg(feature = "alloc")]
owned_ptr_wrapper!(Rc);
#[cfg(feature = "alloc")]
owned_ptr_wrapper!(Arc);
