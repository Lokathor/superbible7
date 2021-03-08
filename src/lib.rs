use core::{convert::TryInto, num::NonZeroU32};

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

  pub const fn loword(w: u32) -> u16 {
    w as u16
  }
  pub const fn hiword(w: u32) -> u16 {
    (w >> 16) as u16
  }

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

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ShaderEnum {
  Compute = GL_COMPUTE_SHADER.0,
  Vertex = GL_VERTEX_SHADER.0,
  TessControl = GL_TESS_CONTROL_SHADER.0,
  TessEval = GL_TESS_EVALUATION_SHADER.0,
  Geometry = GL_GEOMETRY_SHADER.0,
  Fragment = GL_FRAGMENT_SHADER.0,
}
impl ShaderEnum {
  pub fn as_enum(self) -> GLenum {
    GLenum(self as _)
  }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum PolygonEnum {
  Point = GL_POINT.0,
  Line = GL_LINE.0,
  Fill = GL_FILL.0,
}
impl PolygonEnum {
  pub fn as_enum(self) -> GLenum {
    GLenum(self as _)
  }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ShaderID(NonZeroU32);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ProgramID(NonZeroU32);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct VertexArrayID(NonZeroU32);

#[repr(transparent)]
pub struct GlFnsRusty(pub GlFns);
impl core::ops::Deref for GlFnsRusty {
  type Target = GlFns;
  fn deref(&self) -> &GlFns {
    &self.0
  }
}

impl GlFnsRusty {
  /// Clears the color buffer of a given draw buffer to the RGBA given.
  ///
  /// To clear `GL_DRAW_BUFFERi` pass `i` as the `draw_buffer` value. For
  /// example, to clear `GL_DRAW_BUFFER0` you'd pass in a 0.
  pub fn clear_color_draw_buffer(&self, draw_buffer: i32, color: [f32; 4]) {
    unsafe {
      self.ClearBufferfv(GL_COLOR, draw_buffer, color.as_ptr());
    }
  }

  /// Creates a new shader of the given type.
  pub fn create_shader(&self, t: ShaderEnum) -> Option<ShaderID> {
    NonZeroU32::new(self.CreateShader(t.as_enum())).map(ShaderID)
  }

  /// Deletes a shader (or marks it for deletion later).
  pub fn delete_shader(&self, shader: ShaderID) {
    self.DeleteShader(shader.0.get())
  }

  /// Assigns a new source string to the shader.
  ///
  /// Any previous source string is replaced.
  pub fn set_shader_source(&self, shader: ShaderID, src: &str) {
    unsafe {
      self.ShaderSource(
        shader.0.get(),
        1,
        [src.as_ptr()].as_ptr(),
        [src.len().try_into().unwrap()].as_ptr(),
      )
    };
  }

  /// Compiles the shader's source string.
  pub fn compile_shader(&self, shader: ShaderID) {
    self.CompileShader(shader.0.get())
  }

  /// If the shader's last compilation operation worked.
  pub fn get_shader_last_compile_successful(&self, shader: ShaderID) -> bool {
    let mut out = 0;
    unsafe { self.GetShaderiv(shader.0.get(), GL_COMPILE_STATUS, &mut out) };
    out != 0
  }

  /// Gets the info log of the given shader.
  pub fn get_shader_info_log(&self, shader: ShaderID) -> String {
    // capacity needed for the log (including null terminator)
    let mut info_log_length = 0;
    unsafe {
      self.GetShaderiv(shader.0.get(), GL_INFO_LOG_LENGTH, &mut info_log_length)
    };
    if info_log_length == 0 {
      String::new()
    } else {
      let mut v = Vec::with_capacity(info_log_length.try_into().unwrap());
      // printable chars of the log (excludes null terminator)
      let mut printable_byte_count = 0;
      unsafe {
        self.GetShaderInfoLog(
          shader.0.get(),
          v.capacity().try_into().unwrap(),
          &mut printable_byte_count,
          v.as_mut_ptr(),
        );
        v.set_len(printable_byte_count.try_into().unwrap());
      }
      min_alloc_lossy_into_string(v)
    }
  }

  /// Creates a new program object.
  pub fn create_program(&self) -> Option<ProgramID> {
    NonZeroU32::new(self.CreateProgram()).map(ProgramID)
  }

  /// Deletes a program (or marks it for deletion later).
  pub fn delete_program(&self, program: ProgramID) {
    self.DeleteProgram(program.0.get())
  }

  /// Attaches a shader to a program.
  pub fn attach_shader(&self, program: ProgramID, shader: ShaderID) {
    self.AttachShader(program.0.get(), shader.0.get())
  }

  /// Links together all of the program's compiled shader objects.
  pub fn link_program(&self, program: ProgramID) {
    self.LinkProgram(program.0.get())
  }

  /// If the program's last link operation worked.
  pub fn get_program_last_link_successful(&self, program: ProgramID) -> bool {
    let mut out = 0;
    unsafe { self.GetProgramiv(program.0.get(), GL_LINK_STATUS, &mut out) };
    out != 0
  }

  /// Gets the information log for this shader.
  pub fn get_program_info_log(&self, program: ProgramID) -> String {
    // capacity needed for the log (including null terminator)
    let mut info_log_length = 0;
    unsafe {
      self.GetProgramiv(
        program.0.get(),
        GL_INFO_LOG_LENGTH,
        &mut info_log_length,
      )
    };
    if info_log_length == 0 {
      String::new()
    } else {
      let mut v = Vec::with_capacity(info_log_length.try_into().unwrap());
      // printable chars of the log (excludes null terminator)
      let mut printable_byte_count = 0;
      unsafe {
        self.GetProgramInfoLog(
          program.0.get(),
          v.capacity().try_into().unwrap(),
          &mut printable_byte_count,
          v.as_mut_ptr(),
        );
        v.set_len(printable_byte_count.try_into().unwrap());
      }
      min_alloc_lossy_into_string(v)
    }
  }

  pub fn use_program(&self, program: ProgramID) {
    self.UseProgram(program.0.get())
  }

  /// Attempts to create a given number of vertex array objects.
  pub fn create_vertex_arrays<const X: usize>(
    &self,
  ) -> [Option<VertexArrayID>; X] {
    let mut out = [None; X];
    unsafe {
      self.CreateVertexArrays(X.try_into().unwrap(), out.as_mut_ptr().cast())
    };
    out
  }

  /// Deletes the given list of vertex array objects.
  pub fn delete_vertex_arrays<const X: usize>(
    &self, vertex_array_objects: [Option<VertexArrayID>; X],
  ) {
    unsafe {
      self.DeleteVertexArrays(
        X.try_into().unwrap(),
        vertex_array_objects.as_ptr().cast(),
      )
    };
  }

  /// Binds the named vertex array (`Some`), or clears the binding (`None`).
  pub fn bind_vertex_array(&self, opt_array_id: Option<VertexArrayID>) {
    self.BindVertexArray(unsafe { core::mem::transmute(opt_array_id) })
  }

  /// Sets the size of points drawn.
  ///
  /// If `GL_PROGRAM_POINT_SIZE` is enabled, then this will be overridden by the
  /// shader program's `gl_PointSize` value.
  pub fn point_size(&self, size: f32) {
    self.PointSize(size)
  }

  /// Creates a new shader and compiles some source for it.
  ///
  /// On failure, you get the compilation failure log.
  pub fn create_compiled_shader(
    &self, t: ShaderEnum, src: &str,
  ) -> Result<ShaderID, String> {
    let shader = self
      .create_shader(t)
      .ok_or_else(|| String::from("Couldn't create a shader."))?;
    self.set_shader_source(shader, src);
    self.compile_shader(shader);
    if self.get_shader_last_compile_successful(shader) {
      Ok(shader)
    } else {
      let e = Err(self.get_shader_info_log(shader));
      self.delete_shader(shader);
      e
    }
  }

  /// Creates a new program, attaches all named shaders, and links.
  ///
  /// On failure, you get the link error log.
  pub fn create_linked_program(
    &self, shaders: &[ShaderID],
  ) -> Result<ProgramID, String> {
    let program = self
      .create_program()
      .ok_or_else(|| String::from("Couldn't create a program."))?;
    for shader in shaders.iter().copied() {
      self.attach_shader(program, shader);
    }
    self.link_program(program);
    if self.get_program_last_link_successful(program) {
      Ok(program)
    } else {
      let e = Err(self.get_program_info_log(program));
      self.delete_program(program);
      e
    }
  }

  pub fn vertex_attrib_4fv(&self, index: u32, v: [f32; 4]) {
    unsafe { self.VertexAttrib4fv(index, &v) }
  }

  pub fn polygon_mode(&self, mode: PolygonEnum) {
    unsafe { self.PolygonMode(GL_FRONT_AND_BACK, mode.as_enum()) }
  }

  pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
    unsafe { self.Viewport(x, y, width, height) }
  }

  /// Gets a vector of all the extensions supported by the current GL context.
  pub fn get_extension_string(&self) -> Vec<String> {
    let mut num_extensions = 0;
    unsafe { self.GetIntegerv(GL_NUM_EXTENSIONS, &mut num_extensions) };
    let mut v = Vec::with_capacity(num_extensions as usize);
    for ext_num in 0..num_extensions {
      unsafe {
        let p = self.GetStringi(GL_EXTENSIONS, ext_num as u32);
        v.push(min_alloc_lossy_into_string(gather_null_terminated_bytes(p)));
      }
    }
    v
  }
}
