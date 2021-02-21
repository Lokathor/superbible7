use super::*;

#[link(name = "Opengl32")]
extern "system" {
  /// Lookup the function address of a GL-1.2 or later function.
  ///
  /// * Function names are null-terminated strings, and are case-sensitive.
  /// * The extension function addresses are unique for each pixel format.
  /// * All rendering contexts of a given pixel format share the same extension
  ///   function addresses.
  ///
  /// Any GL-1.1 and earlier function *won't* be available through this lookup.
  /// Instead you must use [`GetProcAddress`] with a module handle to
  /// "opengl32.dll".
  ///
  /// MSDN:
  /// [wglGetProcAddress](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglgetprocaddress)
  pub fn wglGetProcAddress(Arg1: LPCSTR) -> PROC;
}

/// As [`wglGetProcAddress`], but with a more Rusty interface.
///
/// * `func_name` should be a byte slice with the desired function's name,
///   *including* the terminating `0`. Use the [`c_str!`] macro for assistance.
///
/// ## Failure
/// * If the slice doesn't end with `0` then this will give an application error
///   without actually calling `wglGetProcAddress`.
/// * If the call to `wglGetProcAddress` fails then you'll get an error code.
///   This will *usually* be `Win32Error(127)`, but other errors are possible.
pub fn wgl_get_proc_address(name: &[u8]) -> Win32Result<NonNull<c_void>> {
  // Safety: check that the slice ends with a 0, as expected.
  match name.last() {
    Some(0) => match unsafe { wglGetProcAddress(name.as_ptr()) } as usize {
      // Some non-zero values can also be errors,
      // https://www.khronos.org/opengl/wiki/Load_OpenGL_Functions#Windows
      1 | 2 | 3 | usize::MAX => Err(get_last_error()),
      proc => NonNull::new(proc as *mut c_void).ok_or_else(|| get_last_error()),
    },
    _ => Err(Win32Error::APP),
  }
}

/// Creates an context for the `HDC` given.
///
/// You'll only get an OpenGL 1.1 context using this function.
///
/// * Set the pixel format of the device context **before** creating a rendering
///   context to go with it.
/// * The new context is **not** automatically made current.
///
/// MSDN:
/// [`wglCreateContext`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglcreatecontext)
pub unsafe fn wgl_create_context(hdc: HDC) -> Win32Result<HGLRC> {
  #[link(name = "Opengl32")]
  extern "system" {
    pub fn wglCreateContext(Arg1: HDC) -> HGLRC;
  }
  let hglrc = wglCreateContext(hdc);
  if hglrc.is_null() {
    Err(get_last_error())
  } else {
    Ok(hglrc)
  }
}

/// Deletes a GL context.
///
/// * You **cannot** use this to delete a context that is current in *another*
///   thread.
/// * You **can** use this to delete a context that's current in *this* thread.
///   The context will automatically be made not-current before it is deleted.
///
/// MSDN:
/// [`wglDeleteContext`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wgldeletecontext)
pub unsafe fn wgl_delete_context(hglrc: HGLRC) -> Win32Result<()> {
  #[link(name = "Opengl32")]
  extern "system" {
    pub fn wglDeleteContext(Arg1: HGLRC) -> BOOL;
  }
  let it_was_deleted = wglDeleteContext(hglrc);
  if it_was_deleted.into() {
    Ok(())
  } else {
    Err(get_last_error())
  }
}

/// Makes a given GL Contest current in this thread.
///
/// If successful, all OpenGL drawing commands in this thread will target the
/// `HDC` given.
///
/// * You can pass `null` as the `HGLRC` value to make any current context
///   become not current. In this case, the `HDC` argument is ignored.
///
/// MSDN:
/// [`wglMakeCurrent`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglmakecurrent)
pub unsafe fn wgl_make_current(hdc: HDC, hglrc: HGLRC) -> Win32Result<()> {
  #[link(name = "Opengl32")]
  extern "system" {
    pub fn wglMakeCurrent(hdc: HDC, hglrc: HGLRC) -> BOOL;
  }
  let it_became_current = wglMakeCurrent(hdc, hglrc);
  if it_became_current.into() {
    Ok(())
  } else {
    Err(get_last_error())
  }
}

// command alias types for the fields of WglExtFns

type wglGetExtensionsStringARB_t =
  extern "system" fn(hdc: HDC) -> *const c_char;

type wglGetPixelFormatAttribivARB_t = unsafe extern "system" fn(
  hdc: HDC,
  iPixelFormat: c_int,
  iLayerPlane: c_int,
  nAttributes: UINT,
  piAttributes: *const c_int,
  piValues: *mut c_int,
) -> BOOL;

type wglGetPixelFormatAttribfvARB_t = unsafe extern "system" fn(
  hdc: HDC,
  iPixelFormat: c_int,
  iLayerPlane: c_int,
  nAttributes: UINT,
  piAttributes: *const c_int,
  pfValues: *mut FLOAT,
) -> BOOL;

type wglChoosePixelFormatARB_t = unsafe extern "system" fn(
  hdc: HDC,
  piAttribIList: *const c_int,
  pfAttribFList: *const FLOAT,
  nMaxFormats: UINT,
  piFormats: *mut c_int,
  nNumFormats: *mut UINT,
) -> BOOL;

/// This holds various function pointers for the `WGL` extensions.
///
/// The reason for this struct will sound slightly silly and almost circular:
/// * Creating a GL context for GL 1.2 or later requires that you use a WGL
///   extension function.
/// * WGL extension function pointers can only be obtained through successful
///   use of `wglGetProcAddress`.
/// * If you use `wglGetProcAddress` to look up a function without a current GL
///   context it will fail.
///
/// **TLDR:** You need to make a GL context just to make a GL context.
///
/// **Solution:** again, this will sound a little silly, but it's true:
/// 1) Use the basic context creation function to make a "dummy" GL context and
///    then set it as current.
/// 2) Use `wglGetProcAddress` to look up all your desired WGL function
///    pointers and store them some place.
/// 3) Clean up the basic GL context.
/// 4) Now you can use your stored WGL functions to make an advanced GL context.
pub struct WglExtFns {
  // WGL_ARB_extensions_string
  wglGetExtensionsStringARB_p: Option<wglGetExtensionsStringARB_t>,
  // WGL_ARB_pixel_format
  wglGetPixelFormatAttribivARB_p: Option<wglGetPixelFormatAttribivARB_t>,
  wglGetPixelFormatAttribfvARB_p: Option<wglGetPixelFormatAttribfvARB_t>,
  wglChoosePixelFormatARB_p: Option<wglChoosePixelFormatARB_t>,
}

impl WglExtFns {
  /// Makes a new `WglExtFns`
  ///
  /// This will make a temporary dummy GL context during creation (if needed),
  /// or use the existing current context if there already is one.
  pub fn new() -> Win32Result<Self> {
    #[link(name = "Opengl32")]
    extern "system" {
      pub fn wglGetCurrentContext() -> HGLRC;
    }
    use core::{
      mem::{transmute, ManuallyDrop},
      ptr::null_mut,
    };
    use utf16_lit::utf16_null;
    struct UnregisterClassWByAtomOnDrop(ATOM);
    impl Drop for UnregisterClassWByAtomOnDrop {
      fn drop(&mut self) {
        if self.0 != 0 {
          let hInstance = HINSTANCE(unsafe { GetModuleHandleW(null()).0 });
          unsafe { UnregisterClassW(self.0 as LPCWSTR, hInstance) };
        }
      }
    }
    struct DestroyWindowOnDrop(HWND);
    impl Drop for DestroyWindowOnDrop {
      fn drop(&mut self) {
        if self.0.is_not_null() {
          unsafe { DestroyWindow(self.0) };
        }
      }
    }
    struct ReleaseDCOnDrop(HDC, HWND);
    impl Drop for ReleaseDCOnDrop {
      fn drop(&mut self) {
        if self.0.is_not_null() {
          unsafe { release_dc(self.1, self.0) };
        }
      }
    }
    struct DeleteContextOnDrop(HGLRC);
    impl Drop for DeleteContextOnDrop {
      fn drop(&mut self) {
        if self.0.is_not_null() {
          let _i_dont_care = unsafe { wgl_delete_context(self.0) };
        }
      }
    }
    struct CleanupDummyGLOnDrop {
      hglrc: ManuallyDrop<DeleteContextOnDrop>,
      hdc: ManuallyDrop<ReleaseDCOnDrop>,
      hwnd: ManuallyDrop<DestroyWindowOnDrop>,
      atom: ManuallyDrop<UnregisterClassWByAtomOnDrop>,
    }
    impl Drop for CleanupDummyGLOnDrop {
      fn drop(&mut self) {
        unsafe {
          ManuallyDrop::drop(&mut self.hglrc);
          ManuallyDrop::drop(&mut self.hdc);
          ManuallyDrop::drop(&mut self.hwnd);
          ManuallyDrop::drop(&mut self.atom);
        }
      }
    }

    let _opt_junk = if unsafe { wglGetCurrentContext() }.is_null() {
      // There's no current context so we need to set one up. This takes a
      // number of steps, and we'll have to carefully clean up after ourselves
      // if there's any problem at any point.
      let hInstance = HINSTANCE(unsafe { GetModuleHandleW(null()).0 });
      let class_name = utf16_null!("TheGLDummyClass_IHopeThisDoesNotClash");
      let wc = WNDCLASSEXW {
        hInstance,
        lpszClassName: class_name.as_ptr(),
        lpfnWndProc: Some(DefWindowProcW),
        style: CS_OWNDC,
        ..WNDCLASSEXW::default()
      };
      let atom = UnregisterClassWByAtomOnDrop(unsafe { RegisterClassExW(&wc) });
      let hwnd = DestroyWindowOnDrop(unsafe {
        CreateWindowExW(
          0,
          class_name.as_ptr(),
          utf16_null!("TheGLDummyWindowTitle").as_ptr(),
          0,
          1,
          1,
          1,
          1,
          HWND::null(),
          HMENU::null(),
          hInstance,
          null_mut(),
        )
      });
      if hwnd.0.is_null() {
        return Err(get_last_error());
      }
      let hdc = ReleaseDCOnDrop(
        unsafe { get_dc(hwnd.0).unwrap_or(HDC::null()) },
        hwnd.0,
      );
      if hdc.0.is_null() {
        return Err(get_last_error());
      }

      let pfd = PIXELFORMATDESCRIPTOR {
        dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
        iPixelType: PFD_TYPE_RGBA,
        cColorBits: 32,
        cDepthBits: 24,
        cStencilBits: 8,
        iLayerType: PFD_MAIN_PLANE,
        ..PIXELFORMATDESCRIPTOR::default()
      };
      let pf_index = choose_pixel_format(hdc.0, &pfd)?;
      set_pixel_format(hdc.0, pf_index, &pfd)?;
      let hglrc = DeleteContextOnDrop(unsafe { wgl_create_context(hdc.0) }?);
      unsafe { wgl_make_current(hdc.0, hglrc.0) }.unwrap();

      Some(CleanupDummyGLOnDrop {
        hglrc: ManuallyDrop::new(hglrc),
        hdc: ManuallyDrop::new(hdc),
        hwnd: ManuallyDrop::new(hwnd),
        atom: ManuallyDrop::new(atom),
      })
    } else {
      // there's already a context set by someone else, so just use
      // `wglGetProcAddress` but *don't* mess with the current context
      // settings.
      None
    };

    // Get the function pointers
    let wglGetExtensionsStringARB_p = unsafe {
      transmute::<Option<NonNull<c_void>>, _>(
        wgl_get_proc_address(c_str!("wglGetExtensionsStringARB")).ok(),
      )
    };
    let wglGetPixelFormatAttribivARB_p = unsafe {
      transmute::<Option<NonNull<c_void>>, _>(
        wgl_get_proc_address(c_str!("wglGetPixelFormatAttribivARB")).ok(),
      )
    };
    let wglGetPixelFormatAttribfvARB_p = unsafe {
      transmute::<Option<NonNull<c_void>>, _>(
        wgl_get_proc_address(c_str!("wglGetPixelFormatAttribfvARB")).ok(),
      )
    };
    let wglChoosePixelFormatARB_p = unsafe {
      transmute::<Option<NonNull<c_void>>, _>(
        wgl_get_proc_address(c_str!("wglChoosePixelFormatARB")).ok(),
      )
    };

    Ok(Self {
      wglGetExtensionsStringARB_p,
      wglGetPixelFormatAttribivARB_p,
      wglGetPixelFormatAttribfvARB_p,
      wglChoosePixelFormatARB_p,
    })
  }
}

// WGL_ARB_extensions_string

impl WglExtFns {
  /// Gets the WGL extensions string.
  ///
  /// The string contains a space-separated list of all WGL extensions that are
  /// supported by the `HDC` given. This information is not context specific,
  /// and you can get the string with no context current.
  ///
  /// Getting the extensions string is *itself* an extension function. If
  /// `WGL_ARB_extensions_string` isn't available then this function won't be
  /// loaded. In that case, you'll get `Ok` with an empty string. At that point,
  /// normal use of `glGetString` might still have extension information you can
  /// query from a current GL context.
  ///
  /// See
  /// [WGL_ARB_extensions_string](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_extensions_string.txt)
  pub fn get_extensions_string_arb(&self, hdc: HDC) -> Win32Result<String> {
    match self.wglGetExtensionsStringARB_p {
      Some(f) => {
        let p = f(hdc);
        if p.is_null() {
          Err(get_last_error())
        } else {
          Ok(min_alloc_lossy_into_string(unsafe {
            gather_null_terminated_bytes(p)
          }))
        }
      }
      None => Ok(String::new()),
    }
  }
}

// WGL_ARB_pixel_format

pub const WGL_NUMBER_PIXEL_FORMATS_ARB: c_int = 0x2000;
pub const WGL_DRAW_TO_WINDOW_ARB: c_int = 0x2001;
pub const WGL_DRAW_TO_BITMAP_ARB: c_int = 0x2002;
pub const WGL_ACCELERATION_ARB: c_int = 0x2003;
pub const WGL_NEED_PALETTE_ARB: c_int = 0x2004;
pub const WGL_NEED_SYSTEM_PALETTE_ARB: c_int = 0x2005;
pub const WGL_SWAP_LAYER_BUFFERS_ARB: c_int = 0x2006;
pub const WGL_SWAP_METHOD_ARB: c_int = 0x2007;
pub const WGL_NUMBER_OVERLAYS_ARB: c_int = 0x2008;
pub const WGL_NUMBER_UNDERLAYS_ARB: c_int = 0x2009;
pub const WGL_TRANSPARENT_ARB: c_int = 0x200A;
pub const WGL_TRANSPARENT_RED_VALUE_ARB: c_int = 0x2037;
pub const WGL_TRANSPARENT_GREEN_VALUE_ARB: c_int = 0x2038;
pub const WGL_TRANSPARENT_BLUE_VALUE_ARB: c_int = 0x2039;
pub const WGL_TRANSPARENT_ALPHA_VALUE_ARB: c_int = 0x203A;
pub const WGL_TRANSPARENT_INDEX_VALUE_ARB: c_int = 0x203B;
pub const WGL_SHARE_DEPTH_ARB: c_int = 0x200C;
pub const WGL_SHARE_STENCIL_ARB: c_int = 0x200D;
pub const WGL_SHARE_ACCUM_ARB: c_int = 0x200E;
pub const WGL_SUPPORT_GDI_ARB: c_int = 0x200F;
pub const WGL_SUPPORT_OPENGL_ARB: c_int = 0x2010;
pub const WGL_DOUBLE_BUFFER_ARB: c_int = 0x2011;
pub const WGL_STEREO_ARB: c_int = 0x2012;
pub const WGL_PIXEL_TYPE_ARB: c_int = 0x2013;
pub const WGL_COLOR_BITS_ARB: c_int = 0x2014;
pub const WGL_RED_BITS_ARB: c_int = 0x2015;
pub const WGL_RED_SHIFT_ARB: c_int = 0x2016;
pub const WGL_GREEN_BITS_ARB: c_int = 0x2017;
pub const WGL_GREEN_SHIFT_ARB: c_int = 0x2018;
pub const WGL_BLUE_BITS_ARB: c_int = 0x2019;
pub const WGL_BLUE_SHIFT_ARB: c_int = 0x201A;
pub const WGL_ALPHA_BITS_ARB: c_int = 0x201B;
pub const WGL_ALPHA_SHIFT_ARB: c_int = 0x201C;
pub const WGL_ACCUM_BITS_ARB: c_int = 0x201D;
pub const WGL_ACCUM_RED_BITS_ARB: c_int = 0x201E;
pub const WGL_ACCUM_GREEN_BITS_ARB: c_int = 0x201F;
pub const WGL_ACCUM_BLUE_BITS_ARB: c_int = 0x2020;
pub const WGL_ACCUM_ALPHA_BITS_ARB: c_int = 0x2021;
pub const WGL_DEPTH_BITS_ARB: c_int = 0x2022;
pub const WGL_STENCIL_BITS_ARB: c_int = 0x2023;
pub const WGL_AUX_BUFFERS_ARB: c_int = 0x2024;
pub const WGL_NO_ACCELERATION_ARB: c_int = 0x2025;
pub const WGL_GENERIC_ACCELERATION_ARB: c_int = 0x2026;
pub const WGL_FULL_ACCELERATION_ARB: c_int = 0x2027;
pub const WGL_SWAP_EXCHANGE_ARB: c_int = 0x2028;
pub const WGL_SWAP_COPY_ARB: c_int = 0x2029;
pub const WGL_SWAP_UNDEFINED_ARB: c_int = 0x202A;
pub const WGL_TYPE_RGBA_ARB: c_int = 0x202B;
pub const WGL_TYPE_COLORINDEX_ARB: c_int = 0x202C;

impl WglExtFns {
  /// Gets info about a pixel format's attributes (integer form).
  ///
  /// * `hdc` The device context for the pixel format.
  /// * `pixel_format_index` the index for the pixel format (1-based, 0 is
  ///   illegal.)
  /// * `layer_plane` the plane of the pixel format. Use 0 for the main plane,
  ///   positive values for overlay planes, and negative values for underlay
  ///   planes.
  /// * `query_attributes` The attributes that you want information for.
  ///
  /// **Output:** An array with size equal to the `query_attributes` size, and
  /// which has all of your answers.
  ///
  /// This returns all queried attributes in *integer* format. If you want
  /// *floating* format results you should use
  /// [get_pixel_format_attrib_fv_arb](Self::get_pixel_format_attrib_fv_arb)
  /// instead.
  ///
  /// See
  /// [wglGetPixelFormatAttribivARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
  ///
  /// ## Failure
  /// * If this function could not be loaded when the `WglExtFns` was created
  ///   you'll get an Application error.
  /// * To check if this function was loaded search for `WGL_ARB_pixel_format`
  ///   in the [extension string](Self::get_extensions_string_arb)
  pub fn get_pixel_format_attrib_iv_arb<const X: usize>(
    &self, hdc: HDC, pixel_format_index: c_int, layer_plane: c_int,
    query_attributes: [c_int; X],
  ) -> Win32Result<[c_int; X]> {
    match self.wglGetPixelFormatAttribivARB_p.as_ref() {
      None => Err(Win32Error::APP),
      Some(f) => {
        let n_attributes = X as _;
        let mut output = [0; X];
        let it_worked = unsafe {
          f(
            hdc,
            pixel_format_index,
            layer_plane,
            n_attributes,
            query_attributes.as_ptr(),
            output.as_mut_ptr(),
          )
        };
        if it_worked.into() {
          Ok(output)
        } else {
          Err(get_last_error())
        }
      }
    }
  }

  /// Gets info about a pixel format's attributes (floating form).
  ///
  /// * `hdc` The device context for the pixel format.
  /// * `pixel_format_index` the index for the pixel format (1-based, 0 is
  ///   illegal.)
  /// * `layer_plane` the plane of the pixel format. Use 0 for the main plane,
  ///   positive values for overlay planes, and negative values for underlay
  ///   planes.
  /// * `query_attributes` The attributes that you want information for.
  ///
  /// **Output:** An array with size equal to the `query_attributes` size, and
  /// which has all of your answers.
  ///
  /// This returns all queried attributes in *floating* format. If you want
  /// *integer* format results you should use
  /// [get_pixel_format_attrib_iv_arb](Self::get_pixel_format_attrib_iv_arb)
  /// instead.
  ///
  /// See
  /// [wglGetPixelFormatAttribivARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
  ///
  /// ## Failure
  /// * If this function could not be loaded when the `WglExtFns` was created
  ///   you'll get an Application error.
  /// * To check if this function was loaded search for `WGL_ARB_pixel_format`
  ///   in the [extension string](Self::get_extensions_string_arb)
  pub fn get_pixel_format_attrib_fv_arb<const X: usize>(
    &self, hdc: HDC, pixel_format_index: c_int, layer_plane: c_int,
    query_attributes: [c_int; X],
  ) -> Win32Result<[c_float; X]> {
    match self.wglGetPixelFormatAttribfvARB_p.as_ref() {
      None => Err(Win32Error::APP),
      Some(f) => {
        let n_attributes = X as _;
        let mut output = [0.0; X];
        let it_worked = unsafe {
          f(
            hdc,
            pixel_format_index,
            layer_plane,
            n_attributes,
            query_attributes.as_ptr(),
            output.as_mut_ptr(),
          )
        };
        if it_worked.into() {
          Ok(output)
        } else {
          Err(get_last_error())
        }
      }
    }
  }

  /// Selects pixel formats that match your requested criteria.
  ///
  /// Prefer this over `wglChoosePixelFormat` (the non-ARB version).
  ///
  /// * `hdc` the device context
  /// * `attrib_int_list` the [key, value] pairs of criteria and integer
  ///   requirement.
  /// * `attrib_float_list` the [key, value] pairs of criteria and floating
  ///   requirement.
  /// * `out_slice` is an output buffer for the results that come back.
  ///
  /// **Output:** If successful, the output is the initial sub-slice of your
  /// `out_slice` buffer that holds pixel formats that match your request.
  /// Formats that are a "better" match are sorted towards the start of the
  /// list. The number of matching formats might be 0 even on success.
  ///
  /// Both attribute request lists contain [key, value] pairs of requirements on
  /// the pixel formats. Each list must be either completely empty or must have
  /// a key of 0 as the last key in the list.
  ///
  /// See
  /// [wglGetPixelFormatAttribivARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
  ///
  /// ## Failure
  /// * If the function was not loaded you will get an application error. Check
  ///   for `WGL_ARB_pixel_format` in the [extension
  ///   string](Self::get_extensions_string_arb).
  /// * If a non-empty attrib list doesn't have a key of 0 as the last
  ///   [key,value] pair this will cause an application error.
  /// * The query itself can also fail. In this case you get a system error and
  ///   your `out_slice` buffer will be rest to all 0s.
  pub fn choose_pixel_format_arb<'out>(
    &self, hdc: HDC, attrib_int_list: &[[c_int; 2]],
    attrib_float_list: &[[FLOAT; 2]], out_slice: &'out mut [c_int],
  ) -> Win32Result<&'out mut [c_int]> {
    match self.wglChoosePixelFormatARB_p.as_ref() {
      None => Err(Win32Error::APP),
      Some(f) => {
        let mut num_formats: UINT = 0;
        let i_ptr: *const i32 = match attrib_int_list.last() {
          Some([k, _v]) => {
            if *k == 0 {
              attrib_int_list.as_ptr().cast()
            } else {
              return Err(Win32Error::APP);
            }
          }
          None => null(),
        };
        let f_ptr: *const f32 = match attrib_float_list.last() {
          Some([k, _v]) => {
            if *k == 0.0 {
              attrib_float_list.as_ptr().cast()
            } else {
              return Err(Win32Error::APP);
            }
          }
          None => null(),
        };
        let it_worked = unsafe {
          f(
            hdc,
            i_ptr,
            f_ptr,
            out_slice.len() as _,
            out_slice.as_mut_ptr(),
            &mut num_formats,
          )
        };
        if it_worked.into() {
          Ok(&mut out_slice[..num_formats as usize])
        } else {
          out_slice.fill(0);
          Err(get_last_error())
        }
      }
    }
  }
}

// TODO: WGL_ARB_create_context

// TODO: WGL_EXT_swap_control

// note that WGL_EXT_swap_control_tear allows extra args!

// TODO: EXT_framebuffer_sRGB (enums)

// TODO: ARB_multisample (enums)

// // // // // // // // // // // // // // // // // // // // // // // // //
// // // // // // // // // // // // // // // // // // // // // // // // //
// // // // // // // // // // // // // // // // // // // // // // // // //
// // // // // // // // // // // // // // // // // // // // // // // // //
// // // // // // // // // // // // // // // // // // // // // // // // //

type wglCreateContextAttribsARB_t = unsafe extern "system" fn(
  hDC: HDC,
  hShareContext: HGLRC,
  attribList: *const c_int,
) -> HGLRC;

type wglSwapIntervalEXT_t = unsafe extern "system" fn(interval: c_int) -> BOOL;

type wglGetSwapIntervalEXT_t = unsafe extern "system" fn() -> c_int;

/// Gets the WGL extension string for the `HDC` passed.
///
/// * This relies on [`wgl_get_proc_address`], and so you must have a context
///   current for it to work.
/// * If `wgl_get_proc_address` fails then an Application Error is generated.
/// * If `wgl_get_proc_address` succeeds but the extension string can't be
///   obtained for some other reason you'll get a System Error.
///
/// The output is a space-separated list of extensions that are supported.
///
/// See
/// [`wglGetExtensionsStringARB`](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_extensions_string.txt)
pub unsafe fn wgl_get_extension_string_arb(
  hdc: HDC,
) -> Result<String, Win32Error> {
  //
  let f: Option<wglGetExtensionsStringARB_t> = core::mem::transmute(
    wgl_get_proc_address(c_str!("wglGetExtensionsStringARB"))?,
  );
  let p: *const u8 =
    (f.ok_or(Win32Error(Win32Error::APPLICATION_ERROR_BIT))?)(hdc).cast();
  if p.is_null() {
    Err(get_last_error())
  } else {
    let bytes = gather_null_terminated_bytes(p);
    Ok(min_alloc_lossy_into_string(bytes))
  }
}

pub struct WglAdvancedFns {
  /// [wglChoosePixelFormatARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
  wglChoosePixelFormatARB_p: wglChoosePixelFormatARB_t,

  /// [wglCreateContextAttribsARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
  wglCreateContextAttribsARB_p: wglCreateContextAttribsARB_t,

  /// [wglSwapIntervalEXT](https://www.khronos.org/registry/OpenGL/extensions/EXT/WGL_EXT_swap_control.txt)
  wglSwapIntervalEXT_p: wglSwapIntervalEXT_t,

  /// [wglGetSwapIntervalEXT](https://www.khronos.org/registry/OpenGL/extensions/EXT/WGL_EXT_swap_control.txt)
  wglGetSwapIntervalEXT_p: wglGetSwapIntervalEXT_t,
}
impl WglAdvancedFns {
  pub unsafe fn for_current_context() -> Result<Self, Win32Error> {
    use core::mem::transmute;
    let wglChoosePixelFormatARB_p = transmute::<NonNull<c_void>, _>(
      wgl_get_proc_address(c_str!("wglChoosePixelFormatARB"))?,
    );
    let wglCreateContextAttribsARB_p = transmute::<NonNull<c_void>, _>(
      wgl_get_proc_address(c_str!("wglCreateContextAttribsARB"))?,
    );
    let wglSwapIntervalEXT_p = transmute::<NonNull<c_void>, _>(
      wgl_get_proc_address(c_str!("wglSwapIntervalEXT"))?,
    );
    let wglGetSwapIntervalEXT_p = transmute::<NonNull<c_void>, _>(
      wgl_get_proc_address(c_str!("wglGetSwapIntervalEXT"))?,
    );
    Ok(Self {
      wglChoosePixelFormatARB_p,
      wglCreateContextAttribsARB_p,
      wglSwapIntervalEXT_p,
      wglGetSwapIntervalEXT_p,
    })
  }
}
impl WglAdvancedFns {
  /// Sets the minimum number of video frame periods per buffer swap for the
  /// window associated with the current context.
  ///
  /// * If `interval` is 0, buffer swaps are not synchronized with the video
  ///   frame timing.
  /// * If `WGL_EXT_swap_control_tear` is available, `interval` can be negative
  ///   to enable adaptive vsync. Otherwise `interval` must be non-negative.
  ///
  /// The default swap interval is 1.
  ///
  /// See
  /// [WGL_EXT_swap_control](https://www.khronos.org/registry/OpenGL/extensions/EXT/WGL_EXT_swap_control.txt)
  /// and
  /// [WGL_EXT_swap_control_tear](https://www.khronos.org/registry/OpenGL/extensions/EXT/WGL_EXT_swap_control_tear.txt)
  pub unsafe fn set_swap_interval(
    &self, interval: c_int,
  ) -> Result<(), Win32Error> {
    let it_worked = (self.wglSwapIntervalEXT_p)(interval);
    if it_worked.into() {
      Ok(())
    } else {
      Err(get_last_error())
    }
  }

  /// Obtains the current swap interval.
  ///
  /// See [`set_swap_interval`](Self::set_swap_interval)
  pub unsafe fn get_swap_interval(&self) -> c_int {
    (self.wglGetSwapIntervalEXT_p)()
  }

  /// Requires [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
  pub fn choose_pixel_format_arb(
    &self, hdc: HDC, int_attrs: &[[c_int; 2]], float_attrs: &[[FLOAT; 2]],
  ) -> Result<c_int, Win32Error> {
    let app_err = Win32Error(Win32Error::APPLICATION_ERROR_BIT);
    let i_ptr = match int_attrs.last() {
      Some([k, _v]) => {
        if *k == 0 {
          int_attrs.as_ptr()
        } else {
          return Err(app_err);
        }
      }
      None => null(),
    };
    let f_ptr = match float_attrs.last() {
      Some([k, _v]) => {
        if *k == 0.0 {
          int_attrs.as_ptr()
        } else {
          return Err(app_err);
        }
      }
      None => null(),
    };
    let mut out_format = 0;
    let mut out_format_count = 0;
    let it_worked = unsafe {
      (self.wglChoosePixelFormatARB_p)(
        hdc,
        i_ptr.cast(),
        f_ptr.cast(),
        1,
        &mut out_format,
        &mut out_format_count,
      )
    };
    if it_worked.into() && out_format_count == 1 {
      Ok(out_format)
    } else {
      Err(get_last_error())
    }
  }

  /// Creates a context that matches the attributes requested.
  ///
  /// * The input slice consists of [key, value] pairs.
  /// * The input slice **can** be empty.
  /// * Any non-empty input must have zero as the key value of the last
  ///   position.
  ///
  /// Requires [WGL_ARB_create_context](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
  pub fn create_context_attribs_arb(
    &self, hdc: HDC, share_context: HGLRC, attribute_list: &[[i32; 2]],
  ) -> Result<HGLRC, Win32Error> {
    let app_err = Win32Error(Win32Error::APPLICATION_ERROR_BIT);
    let i_ptr = match attribute_list.last() {
      Some([k, _v]) => {
        if *k == 0 {
          attribute_list.as_ptr()
        } else {
          return Err(app_err);
        }
      }
      None => null(),
    };
    let hglrc = unsafe {
      (self.wglCreateContextAttribsARB_p)(hdc, share_context, i_ptr.cast())
    };
    if hglrc.is_null() {
      Err(get_last_error())
    } else {
      Ok(hglrc)
    }
  }
}

/// Part of [EXT_framebuffer_sRGB](https://www.khronos.org/registry/OpenGL/extensions/EXT/EXT_framebuffer_sRGB.txt)
pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_EXT: c_int = 0x20A9;

/// Part of [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const WGL_SAMPLE_BUFFERS_ARB: c_int = 0x2041;

/// Part of [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const WGL_SAMPLES_ARB: c_int = 0x2042;

pub const WGL_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
pub const WGL_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
pub const WGL_CONTEXT_LAYER_PLANE_ARB: c_int = 0x2093;
pub const WGL_CONTEXT_FLAGS_ARB: c_int = 0x2094;
pub const WGL_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
pub const WGL_CONTEXT_DEBUG_BIT_ARB: c_int = 0x0001;
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: c_int = 0x0002;
pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x00000001;
pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: c_int = 0x00000002;
