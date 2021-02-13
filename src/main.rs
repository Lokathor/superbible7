#![allow(bad_style)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::ptr::{null, null_mut};
use utf16_lit::utf16_null;

fn main() {
  let hInstance = HINSTANCE(unsafe { GetModuleHandleW(LPCWSTR(null())).0 });
  // TODO: use WNDCLASSEXW / RegisterClassExW instead
  let wc = WNDCLASSW {
    hInstance,
    lpszClassName: LPCWSTR(utf16_null!("TheWindowClass").as_ptr()),
    lpfnWndProc: Some(DefWindowProcW),
    // TODO: fix the handle types
    hCursor: unsafe {
      LoadCursorW(HINSTANCE(HANDLE(PVOID(null_mut()))), IDC_ARROW)
    },
    ..WNDCLASSW::default()
  };
  assert!(unsafe { RegisterClassW(&wc) }.0 != 0);

  // TODO: create the window

  // TODO: handle window messages

  // TODO: closing the window

  // TODO: manage application state
}

use win32::*;
mod win32 {
  use core::ptr::{null, null_mut};

  macro_rules! hardtype {
    ($new:ident($old:ident)) => {
      #[derive(Debug, Clone, Copy, Default)]
      #[repr(transparent)]
      pub struct $new(pub $old);
    };
  }

  type c_int = i32;
  type c_uint = u32;
  type c_char = u8;
  type c_uchar = u8;
  type c_schar = i8;
  type c_short = i16;
  type c_ushort = u16;
  type c_long = i32;
  type c_ulong = u32;
  type c_longlong = i64;
  type c_ulonglong = u64;
  type c_float = f32;
  type c_double = f64;
  type wchar_t = u16;
  //
  pub type LONG_PTR = isize;
  pub type LPARAM = LONG_PTR;
  pub type LRESULT = LONG_PTR;
  pub type UINT = c_uint;
  pub type UINT_PTR = usize;
  pub type WCHAR = wchar_t;
  pub type WPARAM = UINT_PTR;
  pub type WORD = c_ushort;
  pub type ULONG_PTR = usize;
  //
  hardtype!(HANDLE(PVOID));
  hardtype!(HBRUSH(HANDLE));
  hardtype!(HCURSOR(HICON));
  hardtype!(HICON(HANDLE));
  hardtype!(HINSTANCE(HANDLE));
  hardtype!(HMODULE(HANDLE));
  hardtype!(HWND(HANDLE));
  hardtype!(ATOM(WORD));
  //
  type WNDPROC = Option<
    unsafe extern "system" fn(
      hwnd: HWND,
      uMsg: UINT,
      wParam: WPARAM,
      lParam: LPARAM,
    ) -> LRESULT,
  >;

  #[derive(Debug, Clone, Copy)]
  #[repr(transparent)]
  pub struct PVOID(pub *const core::ffi::c_void);
  impl Default for PVOID {
    fn default() -> Self {
      Self(null_mut())
    }
  }

  /// Long Pointer Const Wide String
  #[derive(Debug, Clone, Copy)]
  #[repr(transparent)]
  pub struct LPCWSTR(pub *const WCHAR);
  impl Default for LPCWSTR {
    fn default() -> Self {
      Self(null())
    }
  }

  /// Long Pointer Wide String
  #[derive(Debug, Clone, Copy)]
  #[repr(transparent)]
  pub struct LPWSTR(pub *mut WCHAR);
  impl Default for LPWSTR {
    fn default() -> Self {
      Self(null_mut())
    }
  }

  /// [WNDCLASSW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassw)
  #[repr(C)]
  pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
  }
  impl Default for WNDCLASSW {
    fn default() -> Self {
      unsafe { core::mem::zeroed() }
    }
  }

  #[link(name = "Kernel32")]
  extern "system" {
    /// [DefWindowProcW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(
      hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;

    /// [GetModuleHandleW](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
  }

  #[link(name = "User32")]
  extern "system" {
    /// [LoadCursorW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;

    /// [RegisterClassW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: &WNDCLASSW) -> ATOM;
  }

  /// [MAKEINTRESOURCEW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
  const fn MAKEINTRESOURCEW(i: WORD) -> LPCWSTR {
    LPCWSTR(i as ULONG_PTR as _)
  }
  pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);
}
