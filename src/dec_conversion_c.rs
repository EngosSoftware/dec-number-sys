//! Unsafe bindings for decimal conversion functions.

use crate::{DecContext, DecDouble, DecNumber, DecQuad, DecSingle};

extern "C" {
  /// Unsafe binding to *decimal32ToNumber* function.
  pub fn decimal32ToNumber(ds: *const DecSingle, dn: *mut DecNumber) -> *mut DecNumber;
  /// Unsafe binding to *decimal32FromNumber* function.
  pub fn decimal32FromNumber(ds: *mut DecSingle, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecSingle;
  /// Unsafe binding to *decimal64ToNumber* function.
  pub fn decimal64ToNumber(ds: *const DecDouble, dn: *mut DecNumber) -> *mut DecNumber;
  /// Unsafe binding to *decimal64FromNumber* function.
  pub fn decimal64FromNumber(ds: *mut DecDouble, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecDouble;
  /// Unsafe binding to *decimal128ToNumber* function.
  pub fn decimal128ToNumber(dq: *const DecQuad, dn: *mut DecNumber) -> *mut DecNumber;
  /// Unsafe binding to *decimal128FromNumber* function.
  pub fn decimal128FromNumber(dq: *mut DecQuad, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecQuad;
}
