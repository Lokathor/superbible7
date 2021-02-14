#![allow(dead_code)]

//! Win32 API bindings.

use core::{
  ffi::c_void,
  mem::size_of,
  ptr::{null, null_mut},
};

type c_char = u8;
type c_uchar = u8;
type c_schar = i8;
type c_short = i16;
type c_ushort = u16;
type c_int = i32;
type c_uint = u32;
type c_long = i32;
type c_ulong = u32;
type c_longlong = i64;
type c_ulonglong = u64;
type c_float = f32;
type c_double = f64;
type wchar_t = u16;
type va_list = *mut c_char;

//
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

/// Pointer to Void
pub type PVOID = *mut c_void;

/// "long" Pointer to Void
pub type LPVOID = *mut c_void;

/// "long" Pointer to Const Void
pub type LPCVOID = *const c_void;

/// Long Pointer Const Wide String
pub type LPCWSTR = *const WCHAR;

/// Long Pointer Wide String
pub type LPWSTR = *mut WCHAR;

/// [Window Procedures](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-procedures)
pub type WNDPROC = Option<
  unsafe extern "system" fn(
    hwnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
  ) -> LRESULT,
>;

/// [BOOL](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/9d81be47-232e-42cf-8f0d-7a3b29bf2eb2)
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BOOL(pub u32);
impl From<bool> for BOOL {
  #[must_use]
  fn from(b: bool) -> BOOL {
    Self(b as _)
  }
}
impl From<BOOL> for bool {
  #[must_use]
  fn from(b: BOOL) -> bool {
    b.0 != 0
  }
}
/// The only `false` value for `BOOL`.
pub const FALSE: BOOL = BOOL(false as _);
/// The canonical `true` value for `BOOL`.
pub const TRUE: BOOL = BOOL(true as _);

//
macro_rules! make_handle {
  ($new:ident) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct $new(pub *mut c_void);
    impl Default for $new {
      fn default() -> Self {
        Self::null()
      }
    }
    impl $new {
      pub const fn null() -> Self {
        Self(null_mut())
      }
      pub fn is_null(self) -> bool {
        self.0.is_null()
      }
      pub fn is_not_null(self) -> bool {
        !self.0.is_null()
      }
    }
  };
}

make_handle!(HANDLE);
make_handle!(HBRUSH);
make_handle!(HCURSOR);
make_handle!(HICON);
make_handle!(HINSTANCE);
make_handle!(HMODULE);
make_handle!(HMENU);
make_handle!(HLOCAL);
make_handle!(HWND);

/// [WNDCLASSEXW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw)
///
/// Compared to `WNDCLASSW`, this lets you set a small icon for the window
/// class.
///
/// Also, the `cbSize` value must be the size of this struct. The `Default`
/// instance of this type will handle that for you.
#[repr(C)]
pub struct WNDCLASSEXW {
  pub cbSize: UINT,
  /// [Window Class Styles](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles)
  pub style: UINT,
  /// [Window Procedures](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-procedures)
  pub lpfnWndProc: WNDPROC,
  pub cbClsExtra: c_int,
  pub cbWndExtra: c_int,
  pub hInstance: HINSTANCE,
  pub hIcon: HICON,
  pub hCursor: HCURSOR,
  pub hbrBackground: HBRUSH,
  pub lpszMenuName: LPCWSTR,
  pub lpszClassName: LPCWSTR,
  pub hIconSm: HICON,
}
impl Default for WNDCLASSEXW {
  #[must_use]
  fn default() -> Self {
    WNDCLASSEXW {
      cbSize: size_of::<Self>() as _,
      ..unsafe { core::mem::zeroed() }
    }
  }
}

/// [MSG](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msg)
#[repr(C)]
pub struct MSG {
  pub hwnd: HWND,
  pub message: UINT,
  pub wParam: WPARAM,
  pub lParam: LPARAM,
  pub time: DWORD,
  pub pt: POINT,
  pub lPrivate: DWORD,
}
impl Default for MSG {
  #[must_use]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

/// [POINT](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-point)
#[repr(C)]
pub struct POINT {
  pub x: LONG,
  pub y: LONG,
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

  /// [RegisterClassExW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)
  ///
  /// Compared to `RegisterClassW`, this lets you set a small icon for the
  /// window class.
  pub fn RegisterClassExW(lpWndClass: &WNDCLASSEXW) -> ATOM;

  /// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
  pub fn CreateWindowExW(
    dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR,
    dwStyle: DWORD, X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int,
    hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID,
  ) -> HWND;

  /// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
  pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;

  /// [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
  ///
  /// Note: technically listed as having `BOOL` return value, but actually uses
  /// -1 for "error", 0 for "quit", and other for "other".
  pub fn GetMessageW(
    lpMsg: &mut MSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT,
  ) -> i32;

  /// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
  pub fn TranslateMessage(lpMsg: &MSG) -> BOOL;

  /// [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
  pub fn DispatchMessageW(lpMsg: &MSG) -> LRESULT;

  /// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
  pub fn DestroyWindow(hWnd: HWND) -> BOOL;

  /// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
  pub fn PostQuitMessage(nExitCode: c_int);
}

/// [MAKEINTRESOURCEW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
const fn MAKEINTRESOURCEW(i: WORD) -> LPCWSTR {
  i as ULONG_PTR as _
}

/// ID Cursor: Arrow
pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);

/// Redraw the *full* window if the client area's height changes.
pub const CS_VREDRAW: UINT = 0x0001;

/// Redraw the *full* window if the client area's width changes.
pub const CS_HREDRAW: UINT = 0x0002;

/// Each window of this class gets a unique device context.
pub const CS_OWNDC: UINT = 0x0020;

/// Forces a top-level window onto the taskbar when the window is visible.
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;

/// The window is an overlapped window.
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;

/// The window has a border with a raised edge.
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;

/// The window has a border with a raised edge.
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;

/// The window is an overlapped window.
///
/// Same as the `WS_TILEDWINDOW` style.
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED
  | WS_CAPTION
  | WS_SYSMENU
  | WS_THICKFRAME
  | WS_MINIMIZEBOX
  | WS_MAXIMIZEBOX;

/// The window is an overlapped window.
///
/// An overlapped window has a title bar and a border.
///
/// Same as the `WS_TILED`
/// style.
pub const WS_OVERLAPPED: DWORD = 0x00000000;

/// The window has a title bar (includes the `WS_BORDER` style).
pub const WS_CAPTION: DWORD = 0x00C00000;

/// The window has a window menu on its title bar.
///
/// The `WS_CAPTION` style must
/// also be specified.
pub const WS_SYSMENU: DWORD = 0x00080000;

/// The window has a sizing border.
///
/// Same as the `WS_SIZEBOX` style.
pub const WS_THICKFRAME: DWORD = 0x00040000;

/// The window has a minimize button.
///
/// Cannot be combined with the `WS_EX_CONTEXTHELP` style.
///
/// The `WS_SYSMENU` style must also be specified.
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;

/// The window has a maximize button.
///
/// Cannot be combined with the `WS_EX_CONTEXTHELP` style.
///
/// The `WS_SYSMENU` style must also be specified.
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;

/// Excludes the area occupied by child windows when drawing occurs within the
/// parent window.
///
/// This style is used when creating the parent window.
pub const WS_CLIPCHILDREN: DWORD = 0x02000000;

/// Clips child windows relative to each other.
///
/// In other words, if this window is a child window, then its drawing is
/// clipped by the draw areas of other child windows of the same parent.
pub const WS_CLIPSIBLINGS: DWORD = 0x04000000;

/// Activates the window and displays it in its current size and position.
pub const SW_SHOW: c_int = 5;

/// Sent as a signal that a window or an application should terminate.
///
/// [WM_CLOSE](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-close)
pub const WM_CLOSE: u32 = 0x0010;

/// Sent when a window is being destroyed.
///
/// It is sent to the window procedure of the window being destroyed after the
/// window is removed from the screen.
///
/// [WM_DESTROY](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
pub const WM_DESTROY: u32 = 0x0002;

/// Non-client Create
///
/// [WM_NCCREATE](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_NCCREATE_CONTINUE_CREATION: LRESULT = 1 as _;
pub const WM_NCCREATE_HALT_CREATION: LRESULT = 0 as _;

/// Sent when an application requests that a window be created by calling the
/// CreateWindowEx or CreateWindow function.
///
/// [WM_CREATE](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-create)
pub const WM_CREATE: u32 = 0x0001;
pub const WM_CREATE_CONTINUE_CREATION: LRESULT = 0 as _;
pub const WM_CREATE_HALT_CREATION: LRESULT = -1 as _;

/// Newtype wrapper for a Win32 error code.
///
/// If bit 29 is set, it's an application error instead of a system error.
#[repr(transparent)]
pub struct Win32Error(pub DWORD);
impl Win32Error {
  /// Application errors have the 29th bit set.
  pub const APPLICATION_ERROR_BIT: DWORD = 1 << 29;

  /// Shorthand for a "blank" application error.
  pub const APP: Self = Self(Self::APPLICATION_ERROR_BIT);

  /// Formats the system message of an error code.
  fn format_error_code_system_message(
    &self, f: &mut core::fmt::Formatter,
  ) -> core::fmt::Result {
    #[link(name = "Kernel32")]
    extern "system" {
      /// [`FormatMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
      fn FormatMessageW(
        dwFlags: DWORD, lpSource: LPCVOID, dwMessageId: DWORD,
        dwLanguageId: DWORD, lpBuffer: LPWSTR, nSize: DWORD,
        Arguments: va_list,
      ) -> DWORD;

      /// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
      fn LocalFree(hMem: HLOCAL) -> HLOCAL;
    }
    const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x00000100;
    const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
    const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;

    debug_assert!(self.0 & Self::APPLICATION_ERROR_BIT == 0);

    let dwFlags = FORMAT_MESSAGE_ALLOCATE_BUFFER
      | FORMAT_MESSAGE_FROM_SYSTEM
      | FORMAT_MESSAGE_IGNORE_INSERTS;
    let lpSource = null_mut();
    let dwMessageId = self.0;
    let dwLanguageId = 0;
    // this will point to our allocation after the call
    let mut buffer: *mut u16 = null_mut();
    let lpBuffer = &mut buffer as *mut *mut u16 as *mut u16;
    let nSize = 0;
    let Arguments = null_mut();
    let tchar_count_excluding_null = unsafe {
      FormatMessageW(
        dwFlags,
        lpSource,
        dwMessageId,
        dwLanguageId,
        lpBuffer,
        nSize,
        Arguments,
      )
    };
    if tchar_count_excluding_null == 0 || buffer.is_null() {
      // some sort of problem happened. we can't usefully get_last_error since
      // Display formatting doesn't let you give an error value.
      return Err(core::fmt::Error);
    } else {
      struct OnDropLocalFree(HLOCAL);
      impl Drop for OnDropLocalFree {
        fn drop(&mut self) {
          unsafe { LocalFree(self.0) };
        }
      }
      let _on_drop = OnDropLocalFree(HLOCAL(buffer.cast()));
      let buffer_slice: &[u16] = unsafe {
        core::slice::from_raw_parts(buffer, tchar_count_excluding_null as usize)
      };
      for decode_result in
        core::char::decode_utf16(buffer_slice.iter().copied())
      {
        match decode_result {
          Ok('\r') | Ok('\n') => write!(f, " ")?,
          Ok(ch) => write!(f, "{}", ch)?,
          Err(_) => write!(f, "�")?,
        }
      }
      Ok(())
    }
  }
}
impl std::error::Error for Win32Error {}
macro_rules! impl_fmt_trait_for_Win32Error {
  ($($tr:tt),+ $(,)?) => {
    $(
      impl core::fmt::$tr for Win32Error {
        /// Formats the error code.
        ///
        /// * System errors are "Win32Error({error_code}): system_message"
        ///   * If "alternate" formatting is requested, only
        ///     "Win32Error({error_code})".
        ///   * Any '\r' or '\n' within the system message are converted to a space,
        ///     so "newlines" in the system message will end up being two spaces each.
        /// * Application errors use "Win32Error(Application({error_code}))" with the
        ///   application error bit removed.
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
          if self.0 & Self::APPLICATION_ERROR_BIT > 0 {
            write!(f, "Win32Error(Application(")?;
            core::fmt::$tr::fmt(&(self.0 ^ Self::APPLICATION_ERROR_BIT), f)?;
            write!(f, "))")
          } else {
            write!(f, "Win32Error(")?;
            core::fmt::$tr::fmt(&self.0, f)?;
            write!(f, ")")?;
            if !f.alternate() {
              write!(f, ": ")?;
              self.format_error_code_system_message(f)
            } else {
              Ok(())
            }
          }
        }
      }
    )+
  }
}
impl_fmt_trait_for_Win32Error!(
  Debug, Display, Binary, LowerExp, LowerHex, Octal, UpperExp, UpperHex
);

#[test]
fn test_Win32Error_formatting() {
  let s = format!("{:?}", Win32Error(0));
  assert_eq!("Win32Error(0): The operation completed successfully.  ", s);

  let s = format!("{:#?}", Win32Error(0));
  assert_eq!("Win32Error(0)", s);

  let app_error = format!("{:?}", Win32Error::APP);
  assert_eq!("Win32Error(Application(0))", app_error);
  let app_error =
    format!("{:?}", Win32Error(Win32Error::APPLICATION_ERROR_BIT | 1));
  assert_eq!("Win32Error(Application(1))", app_error);
}

/// Gets the thread-local last-error code value.
///
/// See [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
pub fn get_last_error() -> Win32Error {
  #[link(name = "Kernel32")]
  extern "system" {
    fn GetLastError() -> DWORD;
  }

  Win32Error(unsafe { GetLastError() })
}
