#![allow(dead_code)]

pub use core::ffi::c_void;

pub(super) type c_char = u8;
pub(super) type c_uchar = u8;
pub(super) type c_schar = i8;
pub(super) type c_short = i16;
pub(super) type c_ushort = u16;
pub(super) type c_int = i32;
pub(super) type c_uint = u32;
pub(super) type c_long = i32;
pub(super) type c_ulong = u32;
pub(super) type c_longlong = i64;
pub(super) type c_ulonglong = u64;
pub(super) type c_float = f32;
pub(super) type c_double = f64;
pub(super) type wchar_t = u16;
pub(super) type va_list = *mut c_char;
