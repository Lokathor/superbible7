/// Turns a Rust string slice into a null-terminated utf-16 vector.
pub fn wide_null(s: &str) -> Vec<u16> {
  s.encode_utf16().chain(Some(0)).collect()
}

/// Gathers up the bytes from a pointer.
///
/// The output excludes the null byte.
///
/// ## Safety
/// * The byte sequence must be null-terminated.
pub unsafe fn gather_null_terminated_bytes(mut p: *const u8) -> Vec<u8> {
  let mut v = vec![];
  while *p != 0 {
    v.push(*p);
    p = p.add(1);
  }
  v
}

/// Converts a `Vec<u8>` into a `String` using the minimum amount of
/// re-allocation.
pub fn min_alloc_lossy_into_string(bytes: Vec<u8>) -> String {
  match String::from_utf8(bytes) {
    Ok(s) => s,
    Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
  }
}
