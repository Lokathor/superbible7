use super::*;

impl GlFnsRusty {
  /// Specifies the value of a generic vertex attribute.
  ///
  /// This sets all 4 components of the generic vertex attribute to the values
  /// given.
  ///
  /// * `index`: Specifies the index of the generic vertex attribute to be
  ///   modified. Must be less than `GL_MAX_VERTEX_ATTRIBS`.
  /// * `v`: The values for the attribute.
  ///
  /// See
  /// [glVertexAttrib4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib.xhtml)
  pub fn vertex_attrib_4fv(&self, index: u32, v: [f32; 4]) {
    unsafe { self.VertexAttrib4fv(index, &v) }
  }
}
