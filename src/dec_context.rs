//! Safe bindings for decimal context.

use crate::dec_context_c::*;
use std::ffi::CStr;

/// Initializes context to ANSI X3-274 defaults.
pub const DEC_INIT_BASE: i32 = 0;

/// Initializes context to IEEE 754 defaults, 32-bit.
pub const DEC_INIT_DECIMAL32: i32 = 32;

/// Initializes context to IEEE 754 defaults, 64-bit.
pub const DEC_INIT_DECIMAL64: i32 = 64;

/// Initializes context to IEEE 754 defaults, 128-bit.
pub const DEC_INIT_DECIMAL128: i32 = 128;

/// Decimal context.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct DecContext {
  /// Working precision.
  pub digits: i32,
  /// Maximum positive exponent.
  pub emax: i32,
  /// Minimum negative exponent.
  pub emin: i32,
  /// Rounding mode.
  pub round: u32,
  /// Trap-enabler flags.
  pub traps: u32,
  /// Status flags.
  pub status: u32,
  /// Flag: apply IEEE exponent clamp.
  pub clamp: u8,
}

/// Returns decimal context initialized with maximum specified number of digits.
pub fn dec_context_base(digits: i32) -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, DEC_INIT_BASE);
    context.digits = digits;
    context.traps = 0;
  }
  context
}

/// Returns decimal context initialized for 32-bit decimals.
pub fn dec_context_32() -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, DEC_INIT_DECIMAL32);
    context.traps = 0;
  }
  context
}

/// Returns decimal context initialized for 64-bit decimals.
pub fn dec_context_64() -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, DEC_INIT_DECIMAL64);
    context.traps = 0;
  }
  context
}

/// Returns decimal context initialized for 128-bit decimals.
pub fn dec_context_128() -> DecContext {
  let mut context = DecContext::default();
  unsafe {
    decContextDefault(&mut context, DEC_INIT_DECIMAL128);
    context.traps = 0;
  }
  context
}

/// Safe binding to *decContextDefault* function.
pub fn dec_context_default(kind: i32) -> DecContext {
  unsafe {
    let mut context = DecContext::default();
    decContextDefault(&mut context, kind);
    context.traps = 0;
    context
  }
}

/// Safe binding to *decContextGetStatus* function.
pub fn dec_context_get_status(dc: &mut DecContext) -> u32 {
  unsafe { decContextGetStatus(dc) }
}

/// Safe binding to *decContextStatusToString* function.
pub fn dec_context_status_to_string(dc: &mut DecContext) -> String {
  unsafe {
    let s = decContextStatusToString(dc);
    CStr::from_ptr(s).to_string_lossy().into_owned()
  }
}

/// Safe binding to *decContextZeroStatus* function.
pub fn dec_context_zero_status(dc: &mut DecContext) {
  unsafe {
    decContextZeroStatus(dc);
  }
}
