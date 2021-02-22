#![allow(bad_style)]
#![allow(unused_imports)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use superbible7::*;

use gl46::*;

use core::ptr::{null, null_mut};
use utf16_lit::utf16_null;

use std::time::{Duration, Instant};

#[derive(Default)]
struct WindowData {
  hdc: HDC,
  #[allow(dead_code)]
  hglrc: HGLRC,
  opengl32: HMODULE,
  opt_gl: Option<GlFnsRusty>,
  opt_start: Option<Instant>,
}
impl WindowData {
  // TODO: make the GL crate pass `&[u8]` slices for function loading, not
  // raw pointers.
  unsafe fn gl_get_proc_address(&self, name_ptr: *const u8) -> *mut c_void {
    match wglGetProcAddress(name_ptr) as usize {
      0 | 1 | 2 | 3 | usize::MAX => GetProcAddress(self.opengl32, name_ptr),
      otherwise => otherwise as _,
    }
  }
  pub unsafe fn load_gl_functions(&mut self) {
    assert!(self.opengl32.is_not_null());
    self.opt_gl = Some(GlFnsRusty(
      GlFns::load_from(&|name_ptr| self.gl_get_proc_address(name_ptr)).unwrap(),
    ));
  }
}

fn main() -> Win32Result<()> {
  let hInstance = HINSTANCE(unsafe { GetModuleHandleW(null()).0 });

  let wgl = WglExtFns::new()?;

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

  let hdc = unsafe { get_dc(hwnd).expect("couldn't get the DC!") };

  let ext_string = wgl.get_extensions_string_arb(hdc).unwrap_or(String::new());

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
  let mut pf_index_buffer = [0_i32];
  let pf_indexes = wgl.choose_pixel_format_arb(
    hdc,
    &int_attribs,
    &[],
    &mut pf_index_buffer[..],
  )?;
  let pf_index = pf_indexes[0]; // assume we get at least 1 back.
  let pfd = describe_pixel_format(hdc, pf_index)?;
  set_pixel_format(hdc, pf_index, &pfd)?;

  let flags =
    if cfg!(debug_assertions) { WGL_CONTEXT_DEBUG_BIT_ARB } else { 0 };
  let context_attribs_list = &[
    [WGL_CONTEXT_MAJOR_VERSION_ARB, 4],
    [WGL_CONTEXT_MINOR_VERSION_ARB, 6],
    [WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB],
    [WGL_CONTEXT_FLAGS_ARB, flags],
    [0, 0],
  ];
  let hglrc =
    wgl.create_context_attribs_arb(hdc, HGLRC::null(), context_attribs_list)?;
  unsafe { wgl_make_current(hdc, hglrc) }?;

  // Setup our window data
  unsafe { (*lparam).hdc = hdc };
  unsafe { (*lparam).hglrc = hglrc };
  unsafe { (*lparam).opengl32 = load_library("opengl32.dll")? };
  unsafe { (*lparam).load_gl_functions() };
  unsafe { (*lparam).opt_start = Some(Instant::now()) };

  let gl = unsafe { (*lparam).opt_gl.as_ref().expect("GL was not loaded!") };

  do_the_gl_initialization(gl);

  let mut msg = MSG::default();
  'program: loop {
    // here we poll for messages
    while unsafe {
      PeekMessageW(&mut msg, HWND::null(), 0, 0, PM_REMOVE).into()
    } {
      if msg.message == WM_QUIT {
        break 'program;
      } else {
        unsafe {
          TranslateMessage(&msg);
          DispatchMessageW(&msg);
        }
      }
    }
    // message queue empty, do any other "this frame" type state changes.

    // nothing yet!

    // repaint
    unsafe {
      InvalidateRect(hwnd, None, FALSE);
      UpdateWindow(hwnd);
    };
  }
  Ok(())
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
          let start = if let Some(start) = (*ptr).opt_start {
            start
          } else {
            let start = Instant::now();
            (*ptr).opt_start = Some(start);
            start
          };
          let dur = Instant::now().duration_since(start);
          //println!("paint duration: {:?}", dur);
          do_the_painting(gl, dur);
          SwapBuffers((*ptr).hdc);
          ValidateRect(hwnd, None);
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

/// Does any one-time GL startup.
/// * `gl`: The functions to use for painting.
fn do_the_gl_initialization(gl: &GlFnsRusty) {
  #[cfg(debug_assertions)]
  {
    unsafe {
      gl.Enable(GL_DEBUG_OUTPUT);
      gl.Enable(GL_DEBUG_OUTPUT_SYNCHRONOUS);
      gl.DebugMessageCallback(Some(println_debug_message_callback), null_mut())
    };
  }

  let the_vao = gl.create_vertex_arrays::<1>()[0];
  assert!(the_vao.is_some());
  gl.bind_vertex_array(the_vao);

  let vertex_shader = gl
    .create_compiled_shader(
      ShaderEnum::Vertex,
      "#version 450 core
      void main(void)
      {
        // Declare a hard-coded array of positions
        const vec4 vertices[3] = vec4[3](
          vec4(0.25, -0.25, 0.5, 1.0),
          vec4(-0.25, -0.25, 0.5, 1.0),
          vec4(0.25, 0.25, 0.5, 1.0));
        // Index into our array using gl_VertexID
        gl_Position = vertices[gl_VertexID];
      }",
    )
    .unwrap();

  let frag_shader = gl
    .create_compiled_shader(
      ShaderEnum::Fragment,
      "#version 450 core
      out vec4 color;
      void main(void) {
        color = vec4(0.0, 0.8, 1.0, 1.0);
      }",
    )
    .unwrap();

  let the_program =
    gl.create_linked_program(&[vertex_shader, frag_shader]).unwrap();
  gl.delete_shader(vertex_shader);
  gl.delete_shader(frag_shader);

  gl.use_program(the_program);
  gl.point_size(20.0);
}

/// Does the painting.
/// * `gl`: The functions to use for painting.
/// * `duration`: The duration since the start of the program.
fn do_the_painting(gl: &GlFnsRusty, duration: Duration) {
  let secs_f32 = duration.as_secs_f32();
  let color = [secs_f32.sin() * 0.5, secs_f32.cos() * 0.5, 0.0, 1.0];
  gl.clear_color_draw_buffer(0, color);

  unsafe { gl.DrawArrays(GL_TRIANGLES, 0, 3) };
}
