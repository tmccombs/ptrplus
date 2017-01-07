use std::cell::{Cell, RefCell};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

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
    unsafe fn from_raw(raw: *mut T) -> Self;
}

impl<T> FromRaw<T> for Box<T> {
    unsafe fn from_raw(raw: *mut T) -> Self {
        Box::from_raw(raw)
    }
}

impl FromRaw<c_char> for CString {
    unsafe fn from_raw(raw: *mut c_char) -> CString {
        CString::from_raw(raw)
    }
}

impl<T> FromRaw<T> for *mut T {
    unsafe fn from_raw(raw: *mut T) -> *mut T {
        raw
    }
}

impl<T, U> FromRaw<U> for Option<T> where T: FromRaw<U> {
    unsafe fn from_raw(raw: *mut U) -> Option<T> {
        if raw.is_null() {
            None
        } else {
            Some(T::from_raw(raw))
        }
    }
}


