use super::*;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ProgramID(pub(crate) NonZeroU32);

impl GlFnsRusty {
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
}
