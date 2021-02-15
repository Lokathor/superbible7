#![allow(unused_parens)]

use super::*;

use core::ptr::null_mut;

/// Newtype wrapper for a Win32 error code.
///
/// If bit 29 is set, it's an application error instead of a system error.
#[repr(transparent)]
pub struct Win32Error(pub DWORD);
impl Win32Error {
  /// Application errors have the 29th bit set.
  pub const APPLICATION_ERROR_BIT: DWORD = 1 << 29;

  /// Shorthand for a "blank" application error.
  pub const APP: Self = Self(Self::APPLICATION_ERROR_BIT);

  /// Formats the system message of an error code.
  fn format_error_code_system_message(
    &self, f: &mut core::fmt::Formatter,
  ) -> core::fmt::Result {
    #[link(name = "Kernel32")]
    extern "system" {
      /// [`FormatMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
      fn FormatMessageW(
        dwFlags: DWORD, lpSource: LPCVOID, dwMessageId: DWORD,
        dwLanguageId: DWORD, lpBuffer: LPWSTR, nSize: DWORD,
        Arguments: va_list,
      ) -> DWORD;

      /// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
      fn LocalFree(hMem: HLOCAL) -> HLOCAL;
    }
    /// `FormatMessageW` will allocate a buffer for you
    const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x00000100;
    /// The message that `FormatMessageW` is formatting comes from the system.
    const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
    /// `FormatMessageW` will ignore any string inserts in the message
    /// formatting.
    const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;

    debug_assert!(self.0 & Self::APPLICATION_ERROR_BIT == 0);

    let dwFlags = FORMAT_MESSAGE_ALLOCATE_BUFFER
      | FORMAT_MESSAGE_FROM_SYSTEM
      | FORMAT_MESSAGE_IGNORE_INSERTS;
    let lpSource = null_mut();
    let dwMessageId = self.0;
    let dwLanguageId = 0;
    // this will point to our allocation after the call
    let mut buffer: *mut u16 = null_mut();
    let lpBuffer = &mut buffer as (*mut *mut u16) as (*mut u16);
    let nSize = 0;
    let Arguments = null_mut();
    let tchar_count_excluding_null = unsafe {
      FormatMessageW(
        dwFlags,
        lpSource,
        dwMessageId,
        dwLanguageId,
        lpBuffer,
        nSize,
        Arguments,
      )
    };
    if tchar_count_excluding_null == 0 || buffer.is_null() {
      // some sort of problem happened! core::fmt::Error is a ZST, so we just
      // return it without checking on anything.
      return Err(core::fmt::Error);
    } else {
      struct OnDropLocalFree(HLOCAL);
      impl Drop for OnDropLocalFree {
        fn drop(&mut self) {
          unsafe { LocalFree(self.0) };
        }
      }
      let _on_drop = OnDropLocalFree(HLOCAL(buffer.cast()));
      let buffer_slice: &[u16] = unsafe {
        core::slice::from_raw_parts(buffer, tchar_count_excluding_null as usize)
      };
      for decode_result in
        core::char::decode_utf16(buffer_slice.iter().copied())
      {
        match decode_result {
          Ok('\r') | Ok('\n') => write!(f, " ")?,
          Ok(ch) => write!(f, "{}", ch)?,
          Err(_) => write!(f, "�")?,
        }
      }
      Ok(())
    }
  }
}
impl std::error::Error for Win32Error {}
macro_rules! impl_fmt_trait_for_Win32Error {
  ($($tr:tt),+ $(,)?) => {
    $(
      impl core::fmt::$tr for Win32Error {
        /// Formats the error code.
        ///
        /// * System errors are "Win32Error({error_code}): system_message"
        ///   * If "alternate" formatting is requested, only
        ///     "Win32Error({error_code})".
        ///   * Any '\r' or '\n' within the system message are converted to a space,
        ///     so "newlines" in the system message will end up being two spaces each.
        /// * Application errors use "Win32Error(Application({error_code}))" with the
        ///   application error bit removed.
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
          if self.0 & Self::APPLICATION_ERROR_BIT > 0 {
            write!(f, "Win32Error(Application(")?;
            core::fmt::$tr::fmt(&(self.0 ^ Self::APPLICATION_ERROR_BIT), f)?;
            write!(f, "))")
          } else {
            write!(f, "Win32Error(")?;
            core::fmt::$tr::fmt(&self.0, f)?;
            write!(f, ")")?;
            if !f.alternate() {
              write!(f, ": ")?;
              self.format_error_code_system_message(f)
            } else {
              Ok(())
            }
          }
        }
      }
    )+
  }
}
impl_fmt_trait_for_Win32Error!(
  Debug, Display, Binary, LowerExp, LowerHex, Octal, UpperExp, UpperHex
);

#[test]
fn test_Win32Error_formatting() {
  let s = format!("{:?}", Win32Error(0));
  assert_eq!("Win32Error(0): The operation completed successfully.  ", s);

  let s = format!("{:#?}", Win32Error(0));
  assert_eq!("Win32Error(0)", s);

  let app_error = format!("{:?}", Win32Error::APP);
  assert_eq!("Win32Error(Application(0))", app_error);
  let app_error =
    format!("{:?}", Win32Error(Win32Error::APPLICATION_ERROR_BIT | 1));
  assert_eq!("Win32Error(Application(1))", app_error);
}

/// Gets the thread-local last-error code value.
///
/// See [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
pub fn get_last_error() -> Win32Error {
  #[link(name = "Kernel32")]
  extern "system" {
    fn GetLastError() -> DWORD;
  }

  Win32Error(unsafe { GetLastError() })
}

/// Sets the thread-local last-error code value.
///
/// See [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
pub fn set_last_error(e: Win32Error) {
  #[link(name = "Kernel32")]
  extern "system" {
    fn SetLastError(e: DWORD);
  }

  unsafe { SetLastError(e.0) }
}
