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
