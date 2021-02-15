use gl46::*;

macro_rules! c_str {
  ($text:expr) => {{
    concat!($text, '\0').as_bytes()
  }};
}

pub use win32::*;
pub mod win32 {
  #![allow(bad_style)]
  //! Win32 API bindings.

  use core::{
    mem::{size_of, zeroed},
    ptr::{null, NonNull},
  };

  pub mod boolean;
  pub use boolean::*;

  pub mod c_types;
  pub use c_types::*;

  pub mod constants;
  pub use constants::*;

  pub mod error_code;
  pub use error_code::*;

  pub mod gdi32;
  pub use gdi32::*;

  pub mod handles;
  pub use handles::*;

  pub mod kernel32;
  pub use kernel32::*;

  pub mod opengl32;
  pub use opengl32::*;

  pub mod string_utils;
  pub use string_utils::*;

  pub mod structures;
  pub use structures::*;

  pub mod typedef;
  pub use typedef::*;

  pub mod user32;
  pub use user32::*;
}

pub unsafe extern "system" fn println_debug_message_callback(
  source: GLenum, type_: GLenum, id: u32, severity: GLenum, length: i32,
  message: *const u8, _user_param: *const c_void,
) {
  let src = match source {
    GL_DEBUG_SOURCE_API => "API",
    GL_DEBUG_SOURCE_WINDOW_SYSTEM => "WindowSystem",
    GL_DEBUG_SOURCE_SHADER_COMPILER => "ShaderCompiler",
    GL_DEBUG_SOURCE_THIRD_PARTY => "3rdParty",
    GL_DEBUG_SOURCE_APPLICATION => "App",
    GL_DEBUG_SOURCE_OTHER => "Other",
    _ => "Unknown",
  };
  let ty = match type_ {
    GL_DEBUG_TYPE_ERROR => "Error",
    GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated",
    GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined",
    GL_DEBUG_TYPE_PORTABILITY => "Portability",
    GL_DEBUG_TYPE_PERFORMANCE => "Performance",
    GL_DEBUG_TYPE_MARKER => "Marker",
    GL_DEBUG_TYPE_PUSH_GROUP => "PushGroup",
    GL_DEBUG_TYPE_POP_GROUP => "PopGroup",
    GL_DEBUG_TYPE_OTHER => "Other",
    _ => "Unknown",
  };
  let sev = match severity {
    GL_DEBUG_SEVERITY_HIGH => "High",
    GL_DEBUG_SEVERITY_MEDIUM => "Medium",
    GL_DEBUG_SEVERITY_LOW => "Low",
    GL_DEBUG_SEVERITY_NOTIFICATION => "Note",
    _ => "Unknown",
  };
  let msg = String::from_utf8_lossy(core::slice::from_raw_parts(
    message,
    length as usize,
  ));
  println!(
    "GL>{id} [Src:{src}][Ty:{ty}][Severity:{sev}]> {msg}",
    id = id,
    src = src,
    ty = ty,
    sev = sev,
    msg = msg,
  );
}
