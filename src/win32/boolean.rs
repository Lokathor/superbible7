/// The Win32 "truthy" type.
///
/// We newtype this instead of just a type alias so that we can add a `From`
/// impl for easy conversion to Rust's `bool` type.
///
/// See Also:
/// [BOOL](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/9d81be47-232e-42cf-8f0d-7a3b29bf2eb2)
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BOOL(pub u32);

impl From<bool> for BOOL {
  #[must_use]
  fn from(b: bool) -> BOOL {
    Self(b as _)
  }
}

impl From<BOOL> for bool {
  #[must_use]
  fn from(b: BOOL) -> bool {
    b.0 != 0
  }
}

/// The only `false` value for `BOOL`.
pub const FALSE: BOOL = BOOL(false as _);

/// The canonical `true` value for `BOOL`.
pub const TRUE: BOOL = BOOL(true as _);
