use core::{ffi::c_void, ptr::null_mut};

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
make_handle!(HDC);
make_handle!(HGLRC);
