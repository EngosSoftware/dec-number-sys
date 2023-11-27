//! Unsafe bindings for decimal context.

use crate::DecContext;
use libc::{c_char, c_uint};

extern "C" {
  /// Unsafe binding to *decContextDefault* function.
  pub fn decContextDefault(dc: *mut DecContext, kind: i32) -> *mut DecContext;
  /// Unsafe binding to *decContextGetStatus* function.
  pub fn decContextGetStatus(dc: *mut DecContext) -> c_uint;
  /// Unsafe binding to *decContextStatusToString* function.
  pub fn decContextStatusToString(dc: *mut DecContext) -> *const c_char;
  /// Unsafe binding to *decContextZeroStatus* function.
  pub fn decContextZeroStatus(dc: *mut DecContext) -> *mut DecContext;
}
