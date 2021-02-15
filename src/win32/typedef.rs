use super::*;

pub type LONG = c_long;
pub type LONG_PTR = isize;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type UINT = c_uint;
pub type UINT_PTR = usize;
pub type WCHAR = wchar_t;
pub type WPARAM = UINT_PTR;
pub type WORD = c_ushort;
pub type ULONG_PTR = usize;
pub type DWORD = c_ulong;
pub type ATOM = WORD;
pub type BYTE = u8;
pub type FLOAT = c_float;

/// Pointer to Void
pub type PVOID = *mut c_void;

/// "long" Pointer to Void
pub type LPVOID = *mut c_void;

/// "long" Pointer to Const Void
pub type LPCVOID = *const c_void;

/// Long Pointer Const ANSI String
pub type LPCSTR = *const u8;

/// Long Pointer ANSI String
pub type LPSTR = *mut u8;

/// Long Pointer Const Wide String
pub type LPCWSTR = *const WCHAR;

/// Long Pointer Wide String
pub type LPWSTR = *mut WCHAR;

/// Pointer to a procedure of unknown type.
pub type PROC = *mut c_void;

/// "far" Pointer to a procedure of unknown type.
pub type FARPROC = *mut c_void;

/// [Window Procedures](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-procedures)
pub type WNDPROC = Option<
  unsafe extern "system" fn(
    hwnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
  ) -> LRESULT,
>;
