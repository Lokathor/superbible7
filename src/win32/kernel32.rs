use super::*;

#[link(name = "Kernel32")]
extern "system" {
  /// [DefWindowProcW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
  pub fn DefWindowProcW(
    hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,
  ) -> LRESULT;

  /// [GetModuleHandleW](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
  pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

  /// [`LoadLibraryW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)
  pub fn LoadLibraryW(lpLibFileName: LPCWSTR) -> HMODULE;

  /// [`FreeLibrary`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)
  pub fn FreeLibrary(hLibModule: HMODULE) -> BOOL;

  /// [`GetProcAddress`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)
  pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
}

/// Loads a dynamic library.
///
/// The precise details of how the library is searched for depend on the input
/// string.
///
/// See [`LoadLibraryW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)
pub fn load_library(name: &str) -> Result<HMODULE, Win32Error> {
  let name_null = wide_null(name);
  // Safety: the input pointer is to a null-terminated string
  let hmodule = unsafe { LoadLibraryW(name_null.as_ptr()) };
  if hmodule.is_null() {
    Err(get_last_error())
  } else {
    Ok(hmodule)
  }
}
