use super::*;

#[link(name = "Gdi32")]
extern "system" {
  /// [`DescribePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat)
  fn DescribePixelFormat(
    hdc: HDC, iPixelFormat: c_int, nBytes: UINT,
    ppfd: Option<&mut PIXELFORMATDESCRIPTOR>,
  ) -> c_int;

  /// [`SwapBuffers`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-swapbuffers)
  pub fn SwapBuffers(Arg1: HDC) -> BOOL;
}

/// Gets the maximum pixel format index for the HDC.
///
/// Pixel format indexes are 1-based.
///
/// To print out info on all the pixel formats you'd do something like this:
/// ```no_run
/// # use triangle_from_scratch::win32::*;
/// let hdc = todo!("create a window to get an HDC");
/// let max = unsafe { get_max_pixel_format_index(hdc).unwrap() };
/// for index in 1..=max {
///   let pfd = unsafe { describe_pixel_format(hdc, index).unwrap() };
///   todo!("print the pfd info you want to know");
/// }
/// ```
///
/// See [`DescribePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat)
pub unsafe fn get_max_pixel_format_index(
  hdc: HDC,
) -> Result<c_int, Win32Error> {
  let max_index =
    DescribePixelFormat(hdc, 1, size_of::<PIXELFORMATDESCRIPTOR>() as _, None);
  if max_index == 0 {
    Err(get_last_error())
  } else {
    Ok(max_index)
  }
}

/// Gets the pixel format info for a given pixel format index.
///
/// See [`DescribePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat)
pub fn describe_pixel_format(
  hdc: HDC, format: c_int,
) -> Result<PIXELFORMATDESCRIPTOR, Win32Error> {
  let mut pfd = PIXELFORMATDESCRIPTOR::default();
  let max_index = unsafe {
    DescribePixelFormat(
      hdc,
      format,
      size_of::<PIXELFORMATDESCRIPTOR>() as _,
      Some(&mut pfd),
    )
  };
  if max_index == 0 {
    Err(get_last_error())
  } else {
    Ok(pfd)
  }
}

/// See [`ChoosePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-choosepixelformat)
pub fn choose_pixel_format(
  hdc: HDC, ppfd: &PIXELFORMATDESCRIPTOR,
) -> Result<c_int, Win32Error> {
  #[link(name = "Gdi32")]
  extern "system" {
    pub fn ChoosePixelFormat(hdc: HDC, ppfd: &PIXELFORMATDESCRIPTOR) -> c_int;
  }
  let index = unsafe { ChoosePixelFormat(hdc, ppfd) };
  if index != 0 {
    Ok(index)
  } else {
    Err(get_last_error())
  }
}

/// Sets the pixel format of an HDC.
///
/// * If it's a window's HDC then it sets the pixel format of the window.
/// * You can't set a window's pixel format more than once.
/// * Call this *before* creating an OpenGL context.
/// * OpenGL windows should use [`WS_CLIPCHILDREN`] and [`WS_CLIPSIBLINGS`]
/// * OpenGL windows should *not* use `CS_PARENTDC`
///
/// See [`SetPixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setpixelformat)
pub fn set_pixel_format(
  hdc: HDC, format: c_int, ppfd: &PIXELFORMATDESCRIPTOR,
) -> Result<(), Win32Error> {
  #[link(name = "Gdi32")]
  extern "system" {
    pub fn SetPixelFormat(
      hdc: HDC, format: c_int, ppfd: &PIXELFORMATDESCRIPTOR,
    ) -> BOOL;
  }
  let it_worked = unsafe { SetPixelFormat(hdc, format, ppfd) };
  if it_worked.into() {
    Ok(())
  } else {
    Err(get_last_error())
  }
}
