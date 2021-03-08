use super::*;

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

  /// Sets the size of points drawn.
  ///
  /// If `GL_PROGRAM_POINT_SIZE` is enabled, then this will be overridden by the
  /// shader program's `gl_PointSize` value.
  pub fn point_size(&self, size: f32) {
    self.PointSize(size)
  }

  /// Sets the viewport position and dimensions.
  ///
  /// See [glViewport](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewport.xhtml)
  pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
    unsafe { self.Viewport(x, y, width, height) }
  }

  /// Assigns how all polygons are drawn.
  ///
  /// The default is Fill.
  pub fn polygon_mode(&self, mode: PolygonEnum) {
    unsafe { self.PolygonMode(GL_FRONT_AND_BACK, mode.as_enum()) }
  }

  /// Gets a vector of all the extensions supported by the current GL context.
  pub fn get_all_extension_strings(&self) -> Vec<String> {
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
