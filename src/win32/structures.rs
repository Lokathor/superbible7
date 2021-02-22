use super::*;

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
    WNDCLASSEXW { cbSize: size_of::<Self>() as _, ..unsafe { zeroed() } }
  }
}

/// Contains message information from a thread's message queue.
///
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
#[derive(Debug, Clone, Copy)]
pub struct POINT {
  pub x: LONG,
  pub y: LONG,
}

/// [RECT](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RECT {
  pub left: LONG,
  pub top: LONG,
  pub right: LONG,
  pub bottom: LONG,
}

/// Describes the pixel format of a drawing surface.
///
/// [PIXELFORMATDESCRIPTOR](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-pixelformatdescriptor)
#[repr(C)]
pub struct PIXELFORMATDESCRIPTOR {
  pub nSize: WORD,
  pub nVersion: WORD,
  pub dwFlags: DWORD,
  pub iPixelType: BYTE,
  pub cColorBits: BYTE,
  pub cRedBits: BYTE,
  pub cRedShift: BYTE,
  pub cGreenBits: BYTE,
  pub cGreenShift: BYTE,
  pub cBlueBits: BYTE,
  pub cBlueShift: BYTE,
  pub cAlphaBits: BYTE,
  pub cAlphaShift: BYTE,
  pub cAccumBits: BYTE,
  pub cAccumRedBits: BYTE,
  pub cAccumGreenBits: BYTE,
  pub cAccumBlueBits: BYTE,
  pub cAccumAlphaBits: BYTE,
  pub cDepthBits: BYTE,
  pub cStencilBits: BYTE,
  pub cAuxBuffers: BYTE,
  /// Ignored.
  pub iLayerType: BYTE,
  pub bReserved: BYTE,
  /// Ignored.
  pub dwLayerMask: DWORD,
  pub dwVisibleMask: DWORD,
  /// Ignored.
  pub dwDamageMask: DWORD,
}
impl Default for PIXELFORMATDESCRIPTOR {
  #[must_use]
  fn default() -> Self {
    PIXELFORMATDESCRIPTOR {
      nSize: size_of::<Self>() as _,
      nVersion: 1,
      ..unsafe { zeroed() }
    }
  }
}

/// Defines the initialization parameters passed to the window procedure of an
/// application.
///
/// [CREATESTRUCTW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw)
#[repr(C)]
pub struct CREATESTRUCTW {
  pub lpCreateParams: LPVOID,
  pub hInstance: HINSTANCE,
  pub hMenu: HMENU,
  pub hwndParent: HWND,
  pub cy: c_int,
  pub cx: c_int,
  pub y: c_int,
  pub x: c_int,
  pub style: LONG,
  pub lpszName: LPCWSTR,
  pub lpszClass: LPCWSTR,
  pub dwExStyle: DWORD,
}
