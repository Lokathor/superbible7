use super::*;

#[link(name = "User32")]
extern "system" {
  /// [LoadCursorW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
  pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;

  /// [RegisterClassExW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)
  ///
  /// Compared to `RegisterClassW`, this lets you set a small icon for the
  /// window class.
  pub fn RegisterClassExW(lpWndClass: &WNDCLASSEXW) -> ATOM;

  /// [`UnregisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
  pub fn UnregisterClassW(lpClassName: LPCWSTR, hInstance: HINSTANCE) -> BOOL;

  /// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
  /// * [Extended Window Styles](https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)
  /// * [Window Styles](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles)
  pub fn CreateWindowExW(
    dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR,
    dwStyle: DWORD, X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int,
    hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID,
  ) -> HWND;

  /// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
  pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;

  /// [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
  ///
  /// This is blocking. To do polling instead see [`PeekMessageW`].
  ///
  /// Note: technically listed as having `BOOL` return value, but actually uses
  /// -1 for "error", 0 for "quit", and other for "other".
  pub fn GetMessageW(
    lpMsg: &mut MSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT,
  ) -> i32;

  /// Dispatches incoming sent messages, checks the thread message queue for a
  /// posted message, and retrieves the message (if any exist).
  ///
  /// * `lpMsg`: The location to record the message data.
  /// * `wHnd`: filters the messages based on what window they're intended for.
  ///   * if null: any window message or any thread message.
  ///   * if -1: only thread messages.
  ///   * if a valid handle: only window messages for that window or its
  ///     children.
  ///   * otherwise: error.
  /// * `wMsgFilterMin`: lower bound of message type filter range.
  /// * `wMsgFilterMax`: upper bound of message type filter range.
  /// * `wRemoveMsg`: Specifies how to handle a message.
  ///   * Generally `PM_NOREMOVE` or `PM_REMOVE`, see MSDN.
  ///
  /// **Returns:** if a message is available.
  ///
  /// Message Type Filtering:
  /// * If both filter values are 0 then no message type filtering is applied.
  ///   All message types will be delivered.
  /// * The `WM_QUIT` message will always be delivered, regardless of filter
  ///   arguments.
  ///
  /// MSDN:
  /// [`PeekMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
  pub fn PeekMessageW(
    lpMsg: &mut MSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT,
    wRemoveMsg: UINT,
  ) -> BOOL;

  /// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
  pub fn TranslateMessage(lpMsg: &MSG) -> BOOL;

  /// [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
  pub fn DispatchMessageW(lpMsg: &MSG) -> LRESULT;

  /// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
  pub fn DestroyWindow(hWnd: HWND) -> BOOL;

  /// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
  pub fn PostQuitMessage(nExitCode: c_int);

  /// Changes the text of the specified window's title bar, if any.
  ///
  /// If the specified window is a control, the text of the control is changed.
  ///
  /// [`SetWindowTextW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
  pub fn SetWindowTextW(hWnd: HWND, lpString: LPCWSTR) -> BOOL;

  /// [`GetDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
  pub fn GetDC(hWnd: HWND) -> HDC;

  /// [`ReleaseDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
  pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;

  /// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
  pub fn SetWindowLongPtrW(
    hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR,
  ) -> LONG_PTR;

  /// [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
  pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;

  /// Updates the client area of the specified window by sending a [`WM_PAINT`]
  /// message to the window if the window's update region is not empty.
  ///
  /// The function sends a [`WM_PAINT`] message directly to the window procedure
  /// of the specified window, bypassing the application queue.
  ///
  /// If the update region is empty, no message is sent.
  ///
  /// MSDN: [`UpdateWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow)
  pub fn UpdateWindow(hwnd: HWND) -> BOOL;
}

/// Un-registers the window class from the `HINSTANCE` given.
///
/// * The name must be the name of a registered window class.
/// * This requires re-encoding the name to null-terminated utf-16, which
///   allocates. Using [`unregister_class_by_atom`] instead does not allocate,
///   if you have the atom available.
/// * Before calling this function, an application must destroy all windows
///   created with the specified class.
///
/// See
/// [`UnregisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
pub unsafe fn unregister_class_by_name(
  name: &str, instance: HINSTANCE,
) -> Result<(), Win32Error> {
  let name_null = wide_null(name);
  let it_was_unregistered = UnregisterClassW(name_null.as_ptr(), instance);
  if it_was_unregistered.into() {
    Ok(())
  } else {
    Err(get_last_error())
  }
}

/// Un-registers the window class from the `HINSTANCE` given.
///
/// * The atom must be the atom of a registered window class.
/// * Before calling this function, an application must destroy all windows
///   created with the specified class.
///
/// See [`UnregisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
pub unsafe fn unregister_class_by_atom(
  a: ATOM, instance: HINSTANCE,
) -> Result<(), Win32Error> {
  let it_was_unregistered = UnregisterClassW(a as LPCWSTR, instance);
  if it_was_unregistered.into() {
    Ok(())
  } else {
    Err(get_last_error())
  }
}

/// See [`GetDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
pub unsafe fn get_dc(hwnd: HWND) -> Option<HDC> {
  let hdc = GetDC(hwnd);
  if hdc.is_null() {
    None
  } else {
    Some(hdc)
  }
}

/// See [`ReleaseDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
#[must_use]
pub unsafe fn release_dc(hwnd: HWND, hdc: HDC) -> bool {
  let was_released = ReleaseDC(hwnd, hdc);
  was_released != 0
}

/// See [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
pub unsafe fn destroy_window(hwnd: HWND) -> Result<(), Win32Error> {
  let it_was_destroyed = DestroyWindow(hwnd);
  if it_was_destroyed.into() {
    Ok(())
  } else {
    Err(get_last_error())
  }
}

/// Sets the "userdata" pointer of the window (`GWLP_USERDATA`).
///
/// **Returns:** The previous userdata pointer.
///
/// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
pub unsafe fn set_window_userdata<T>(
  hwnd: HWND, ptr: *mut T,
) -> Result<*mut T, Win32Error> {
  set_last_error(Win32Error(0));
  let out = SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr as LONG_PTR);
  if out == 0 {
    // if output is 0, it's only a "real" error if the last_error is non-zero
    let last_error = get_last_error();
    if last_error.0 != 0 {
      Err(last_error)
    } else {
      Ok(out as *mut T)
    }
  } else {
    Ok(out as *mut T)
  }
}

/// Gets the "userdata" pointer of the window (`GWLP_USERDATA`).
///
/// **Returns:** The userdata pointer.
///
/// [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
pub unsafe fn get_window_userdata<T>(hwnd: HWND) -> Result<*mut T, Win32Error> {
  set_last_error(Win32Error(0));
  let out = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
  if out == 0 {
    // if output is 0, it's only a "real" error if the last_error is non-zero
    let last_error = get_last_error();
    if last_error.0 != 0 {
      Err(last_error)
    } else {
      Ok(out as *mut T)
    }
  } else {
    Ok(out as *mut T)
  }
}
