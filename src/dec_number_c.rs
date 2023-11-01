//! Unsafe bindings for arbitrary precision decimal.

use crate::{DecContext, DecNumber};
use libc::{c_char, c_int, c_uchar, c_uint};

#[rustfmt::skip]
extern "C" {
  /// Unsafe binding to *decNumberAdd* function.
  pub fn decNumberAdd(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberCompare* function.
  pub fn decNumberCompare(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberDivide* function.
  pub fn decNumberDivide(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberExp* function.
  pub fn decNumberExp(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberFromInt32* function.
  pub fn decNumberFromInt32(dn: *mut DecNumber, n: c_int) -> *mut DecNumber;
  /// Unsafe binding to *decNumberFromString* function.
  pub fn decNumberFromString(dn: *mut DecNumber, s: *const c_char, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberFromUInt32* function.
  pub fn decNumberFromUInt32(dn: *mut DecNumber, n: c_uint) -> *mut DecNumber;
  /// Unsafe binding to *decNumberLn* function.
  pub fn decNumberLn(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberMinus* function.
  pub fn decNumberMinus(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberMultiply* function.
  pub fn decNumberMultiply(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberPlus* function.
  pub fn decNumberPlus(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberPower* function.
  pub fn decNumberPower(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberQuantize* function.
  pub fn decNumberQuantize(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberReduce* function.
  pub fn decNumberReduce(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberRescale* function.
  pub fn decNumberRescale(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberScaleB* function.
  pub fn decNumberScaleB(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberSetBCD* function.
  pub fn decNumberSetBCD(dn: *mut DecNumber, bcd: *const c_uchar, n: c_uint) -> *mut DecNumber;
  /// Unsafe binding to *decNumberSquareRoot* function.
  pub fn decNumberSquareRoot(dn: *mut DecNumber, dn1: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberSubtract* function.
  pub fn decNumberSubtract(dn: *mut DecNumber, dn1: *const DecNumber, dn2: *const DecNumber, dc: *mut DecContext) -> *mut DecNumber;
  /// Unsafe binding to *decNumberToString* function.
  pub fn decNumberToString(dn1: *const DecNumber, s: *mut c_char) -> *mut c_char;
  /// Unsafe binding to *decNumberZero* function.
  pub fn decNumberZero(dn: *mut DecNumber);
}
