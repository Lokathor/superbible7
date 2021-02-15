#![allow(bad_style)]
#![allow(unused_imports)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use superbible7::*;

use gl46::*;

use core::ptr::{null, null_mut};
use utf16_lit::utf16_null;

#[derive(Default)]
struct WindowData {
  hdc: HDC,
  #[allow(dead_code)]
  hglrc: HGLRC,
  opengl32: HMODULE,
  opt_gl: Option<GlFns>,
}
impl WindowData {
  unsafe fn gl_get_proc_address(&self, name_ptr: *const u8) -> *mut c_void {
    match wglGetProcAddress(name_ptr) as usize {
      0 | 1 | 2 | 3 | usize::MAX => GetProcAddress(self.opengl32, name_ptr),
      otherwise => otherwise as _,
    }
  }
  pub unsafe fn load_gl_functions(&mut self) {
    self.opt_gl = Some(
      GlFns::load_from(&|name_ptr| self.gl_get_proc_address(name_ptr)).unwrap(),
    );
  }
}

fn main() {
  let hInstance = HINSTANCE(unsafe { GetModuleHandleW(null()).0 });

  let (ext_string, wgl_fns) = {
    let wc = WNDCLASSEXW {
      hInstance,
      lpszClassName: utf16_null!("TheGLDummyClass").as_ptr(),
      lpfnWndProc: Some(DefWindowProcW),
      style: CS_OWNDC,
      ..WNDCLASSEXW::default()
    };
    let atom = unsafe { RegisterClassExW(&wc) };
    assert!(atom != 0);

    let hwnd = unsafe {
      CreateWindowExW(
        0,
        atom as LPCWSTR,
        utf16_null!("TheGLDummyWindow").as_ptr(),
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
    };
    assert!(hwnd.is_not_null(), "CreateWindowError: {}", get_last_error());

    let hdc = unsafe { get_dc(hwnd) }.unwrap();

    let pfd = PIXELFORMATDESCRIPTOR {
      dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
      iPixelType: PFD_TYPE_RGBA,
      cColorBits: 32,
      cDepthBits: 24,
      cStencilBits: 8,
      iLayerType: PFD_MAIN_PLANE,
      ..PIXELFORMATDESCRIPTOR::default()
    };
    let pf_index = choose_pixel_format(hdc, &pfd).unwrap();
    set_pixel_format(hdc, pf_index, &pfd).unwrap();

    let hglrc = unsafe { wgl_create_context(hdc) }.unwrap();
    unsafe { wgl_make_current(hdc, hglrc) }.unwrap();

    let ext_string = unsafe { wgl_get_extension_string_arb(hdc) }.unwrap();
    print!(">> WGL Extensions Available: ");
    for (i, ext) in ext_string.split(' ').filter(|s| !s.is_empty()).enumerate()
    {
      print!("{}{}", if i > 0 { ", " } else { "" }, ext);
    }
    println!();
    assert!(ext_string.contains("WGL_ARB_pixel_format"));
    assert!(ext_string.contains("WGL_ARB_create_context"));

    let wgl_fns = unsafe { WglAdvancedFns::for_current_context() }.unwrap();

    unsafe { wgl_delete_context(hglrc) }.unwrap();
    assert!(unsafe { release_dc(hwnd, hdc) });
    unsafe { destroy_window(hwnd) }.unwrap();

    (ext_string, wgl_fns)
  };

  let wc = WNDCLASSEXW {
    hInstance,
    lpszClassName: utf16_null!("TheWindowClass").as_ptr(),
    lpfnWndProc: Some(window_procedure),
    hCursor: unsafe { LoadCursorW(HINSTANCE::null(), IDC_ARROW) },
    style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
    ..WNDCLASSEXW::default()
  };
  let atom = unsafe { RegisterClassExW(&wc) };
  assert!(atom != 0);

  let lparam: *mut WindowData = Box::leak(Box::new(WindowData::default()));
  let hwnd = unsafe {
    CreateWindowExW(
      WS_EX_APPWINDOW | WS_EX_OVERLAPPEDWINDOW,
      atom as LPCWSTR,
      utf16_null!("SuperBible7").as_ptr(),
      WS_VISIBLE | WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_CLIPSIBLINGS,
      50,
      50,
      800,
      600,
      HWND::null(),
      HMENU::null(),
      hInstance,
      lparam.cast(),
    )
  };
  assert!(hwnd.is_not_null());

  let hdc = unsafe { get_dc(hwnd) }.unwrap();

  // base criteria
  let mut int_attribs = vec![
    [WGL_DRAW_TO_WINDOW_ARB, true as _],
    [WGL_SUPPORT_OPENGL_ARB, true as _],
    [WGL_DOUBLE_BUFFER_ARB, true as _],
    [WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB],
    [WGL_COLOR_BITS_ARB, 32],
    [WGL_DEPTH_BITS_ARB, 24],
    [WGL_STENCIL_BITS_ARB, 8],
  ];
  if ext_string.contains("WGL_EXT_framebuffer_sRGB") {
    int_attribs.push([WGL_FRAMEBUFFER_SRGB_CAPABLE_EXT, true as _]);
  };
  if ext_string.contains("WGL_ARB_multisample") {
    int_attribs.push([WGL_SAMPLE_BUFFERS_ARB, 1]);
  };
  int_attribs.push([0, 0]);
  let pf_index =
    wgl_fns.choose_pixel_format_arb(hdc, &int_attribs, &[]).unwrap();
  let pfd = describe_pixel_format(hdc, pf_index).unwrap();
  set_pixel_format(hdc, pf_index, &pfd).unwrap();

  let flags =
    if cfg!(debug_assertions) { WGL_CONTEXT_DEBUG_BIT_ARB } else { 0 };
  let context_attribs_list = &[
    [WGL_CONTEXT_MAJOR_VERSION_ARB, 4],
    [WGL_CONTEXT_MINOR_VERSION_ARB, 6],
    [WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB],
    [WGL_CONTEXT_FLAGS_ARB, flags],
    [0, 0],
  ];
  let hglrc = wgl_fns
    .create_context_attribs_arb(hdc, HGLRC::null(), context_attribs_list)
    .unwrap();
  unsafe { wgl_make_current(hdc, hglrc) }.unwrap();

  // Setup our window data
  unsafe { (*lparam).hdc = hdc };
  unsafe { (*lparam).hglrc = hglrc };
  unsafe { (*lparam).opengl32 = load_library("opengl32.dll").unwrap() };
  unsafe { (*lparam).load_gl_functions() };

  #[cfg(debug_assertions)]
  if let Some(gl) = unsafe { (*lparam).opt_gl.as_ref() } {
    unsafe {
      gl.Enable(GL_DEBUG_OUTPUT);
      gl.Enable(GL_DEBUG_OUTPUT_SYNCHRONOUS);
      gl.DebugMessageCallback(Some(println_debug_message_callback), null_mut())
    };
  }

  let mut msg = MSG::default();
  loop {
    match unsafe { GetMessageW(&mut msg, HWND::null(), 0, 0) } {
      -1 => {
        println!("`GetMessageW` error: {}", get_last_error());
        break;
      }
      0 => break,
      _other => unsafe {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
      },
    }
  }
}

pub unsafe extern "system" fn window_procedure(
  hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM,
) -> LRESULT {
  match msg {
    WM_NCCREATE => {
      println!("Non-client Create");
      DefWindowProcW(hwnd, msg, w_param, l_param);
      let createstruct = l_param as *mut CREATESTRUCTW;
      if createstruct.is_null() {
        return WM_NCCREATE_HALT_CREATION;
      }
      let ptr = (*createstruct).lpCreateParams as *mut WindowData;
      if set_window_userdata::<WindowData>(hwnd, ptr).is_ok() {
        return WM_NCCREATE_CONTINUE_CREATION;
      } else {
        return WM_NCCREATE_HALT_CREATION;
      }
    }
    WM_CREATE => {
      println!("Create");
      return WM_CREATE_CONTINUE_CREATION;
    }
    WM_PAINT => match get_window_userdata::<WindowData>(hwnd) {
      Ok(ptr) if !ptr.is_null() => {
        if let Some(gl) = (*ptr).opt_gl.as_ref() {
          do_the_painting(gl);
          SwapBuffers((*ptr).hdc);
        } else {
          println!("WM_PAINT, but GL not loaded.");
        }
      }
      _otherwise => {
        println!("WM_PAINT, but no userdata pointer found.");
      }
    },
    WM_CLOSE => {
      println!("Close");
      if let Err(e) = destroy_window(hwnd) {
        println!("Error While Destroying The Window: {}", e);
      }
    }
    WM_DESTROY => {
      println!("Destroy");
      PostQuitMessage(0);
    }
    _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
  }
  0
}

unsafe fn do_the_painting(gl: &GlFns) {
  let red = [1.0, 0.0, 0.0, 1.0];
  gl.ClearBufferfv(GL_COLOR, 0, red.as_ptr());
}
