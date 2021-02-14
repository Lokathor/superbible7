#![allow(bad_style)]
#![allow(unused_imports)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::ptr::{null, null_mut};
use utf16_lit::utf16_null;

pub mod win32;
use win32::*;

fn main() {
  let hInstance = HINSTANCE(unsafe { GetModuleHandleW(null()).0 });
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
      null_mut(),
    )
  };
  assert!(hwnd.is_not_null());

  //let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

  let mut msg = MSG::default();
  loop {
    match unsafe { GetMessageW(&mut msg, HWND::null(), 0, 0) } {
      -1 => {
        panic!("`GetMessageW` error: {}", get_last_error())
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
      return WM_NCCREATE_CONTINUE_CREATION;
    }
    WM_CREATE => {
      println!("Create");
      return WM_CREATE_CONTINUE_CREATION;
    }
    WM_CLOSE => drop(DestroyWindow(hwnd)),
    WM_DESTROY => PostQuitMessage(0),
    _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
  }
  0
}
