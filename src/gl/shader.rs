use super::*;

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
#[repr(transparent)]
pub struct ShaderID(pub(crate) NonZeroU32);

impl GlFnsRusty {
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
}
