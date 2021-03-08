use super::*;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct VertexArrayID(pub(crate) NonZeroU32);

impl GlFnsRusty {
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
}
