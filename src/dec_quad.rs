//! Safe bindings for 128-bit decimal.

use crate::dec_context::DecContext;
use crate::dec_quad_c::*;
use crate::{DEC_FLOAT_SIGN, DEC_QUAD_PMAX};
use libc::{c_char, c_int, c_uint};
use std::ffi::{CStr, CString};
use std::fmt::Debug;

/// Length in bytes of the [DecQuad] union.
pub const DEC_QUAD_BYTES: usize = 16;

/// Maximum length of the [DecQuad] string.
pub const DEC_QUAD_STRING: usize = 43;

/// [DecQuad] string buffer.
pub const DEC_QUAD_STRING_BUFFER: [c_char; DEC_QUAD_STRING] = [0; DEC_QUAD_STRING];

/// [DecQuad] value `0` (zero).
#[rustfmt::skip]
pub const DEC_QUAD_ZERO: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] }}
};

/// [DecQuad] value `1` (one).
#[rustfmt::skip]
pub const DEC_QUAD_ONE: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01] }}
};

/// [DecQuad] value `2` (two).
#[rustfmt::skip]
pub const DEC_QUAD_TWO: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02] }}
};

/// [DecQuad] value `10` (ten).
#[rustfmt::skip]
pub const DEC_QUAD_TEN: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10] }}
};

/// [DecQuad] value `100` (hundred).
#[rustfmt::skip]
pub const DEC_QUAD_HUNDRED: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80] }}
};

/// [DecQuad] value `1000` (thousand).
#[rustfmt::skip]
pub const DEC_QUAD_THOUSAND: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x04, 0x00] }}
};

/// [DecQuad] value `1000000` (million).
#[rustfmt::skip]
pub const DEC_QUAD_MILLION: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00] }}
};

/// [DecQuad] value `1000000000` (billion).
#[rustfmt::skip]
pub const DEC_QUAD_BILLION: DecQuad = {
  #[cfg(target_endian = "little")]
  { DecQuad { bytes: [0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x22] }}
  #[cfg(target_endian = "big")]
  { DecQuad { bytes: [0x22, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00] }}
};

/// 128-bit decimal number.
#[repr(C)]
#[derive(Copy, Clone)]
pub union DecQuad {
  pub bytes: [u8; DEC_QUAD_BYTES],
  pub shorts: [u16; DEC_QUAD_BYTES / 2],
  pub words: [u32; DEC_QUAD_BYTES / 4],
  pub longs: [u64; DEC_QUAD_BYTES / 8],
}

impl Default for DecQuad {
  /// The default value for [DecQuad] is positive zero.
  fn default() -> Self {
    DEC_QUAD_ZERO
  }
}

impl Debug for DecQuad {
  /// Converts [DecQuad] to a string in the form of hexadecimal bytes separated with spaces.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]",
      (0..DEC_QUAD_BYTES)
        .rev()
        .fold("".to_string(), |s, i| format!("{} {:02X}", s, unsafe { self.bytes[i] }))
        .trim_start()
    )
  }
}

/// Safe binding to *decQuadAbs* function.
pub fn dec_quad_abs(dq1: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadAbs(&mut dq, dq1, dc);
  }
  dq
}

/// Safe binding to *decQuadAdd* function.
pub fn dec_quad_add(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadAdd(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadAnd* function.
pub fn dec_quad_and(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadAnd(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadCanonical* function.
pub fn dec_quad_canonical(dq1: &DecQuad) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadCanonical(&mut dq, dq1);
  }
  dq
}

/// Safe binding to *decQuadCompare* function.
pub fn dec_quad_compare(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadCompare(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadCompareSignal* function.
pub fn dec_quad_compare_signal(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadCompareSignal(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadCompareTotal* function.
pub fn dec_quad_compare_total(dq1: &DecQuad, dq2: &DecQuad) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadCompareTotal(&mut dq, dq1, dq2);
  }
  dq
}

/// Safe binding to *decQuadCompareTotalMag* function.
pub fn dec_quad_compare_total_mag(dq1: &DecQuad, dq2: &DecQuad) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadCompareTotalMag(&mut dq, dq1, dq2);
  }
  dq
}

/// Safe binding to *decQuadCopy* function.
pub fn dec_quad_copy(dq1: &DecQuad) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadCopy(&mut dq, dq1);
  }
  dq
}

/// Safe binding to *decQuadCopyAbs* function.
pub fn dec_quad_copy_abs(dq1: &DecQuad) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadCopyAbs(&mut dq, dq1);
  }
  dq
}

/// Safe binding to *decQuadCopyNegate* function.
pub fn dec_quad_copy_negate(dq1: &DecQuad) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadCopyNegate(&mut dq, dq1);
  }
  dq
}

/// Safe binding to *decQuadCopySign* function.
pub fn dec_quad_copy_sign(dq1: &DecQuad, dq2: &DecQuad) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadCopySign(&mut dq, dq1, dq2);
  }
  dq
}

/// Safe binding to *decQuadDigits* function.
pub fn dec_quad_digits(dq1: &DecQuad) -> c_uint {
  let dq;
  unsafe {
    dq = decQuadDigits(dq1);
  }
  dq
}

/// Safe binding to *decQuadDivide* function.
pub fn dec_quad_divide(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadDivide(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadDivideInteger* function.
pub fn dec_quad_divide_integer(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadDivideInteger(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadFMA* function.
pub fn dec_quad_fma(dq1: &DecQuad, dq2: &DecQuad, dq3: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadFMA(&mut dq, dq1, dq2, dq3, dc);
  }
  dq
}

/// Safe binding to *decQuadFromBCD* function.
pub fn dec_quad_from_bcd(bcd: &[u8; DEC_QUAD_PMAX], exp: i32, sign: bool) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadFromBCD(&mut dq, exp, bcd.as_ptr(), if sign { DEC_FLOAT_SIGN } else { 0 });
  }
  dq
}

/// Safe binding to *decQuadFromInt32* function.
pub fn dec_quad_from_i32(n: i32) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadFromInt32(&mut dq, n);
  }
  dq
}

// /// Safe binding to *decQuadFromNumber* function.
// pub fn dec_quad_from_number(dn: &DecNumber, dc: &mut DecContext) -> DecQuad {
//   let mut dq = DEC_QUAD_ZERO;
//   unsafe {
//     decQuadFromNumber(&mut dq, dn, dc);
//   }
//   dq
// }

// /// Safe binding to *decQuadFromPacked* function.
// pub fn dec_quad_from_packed(exp: i32, pack: &[u8; DEC_QUAD_PMAX]) -> DecQuad {
//   let mut dq = DEC_QUAD_ZERO;
//   unsafe {
//     decQuadFromPacked(&mut dq, exp, pack.as_ptr());
//   }
//   dq
// }

/// Safe binding to *decQuadFromString* function.
pub fn dec_quad_from_string(s: &str, dc: &mut DecContext) -> DecQuad {
  let c_s = CString::new(s).unwrap();
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadFromString(&mut dq, c_s.as_ptr(), dc);
  }
  dq
}

/// Safe binding to *decQuadFromUInt32* function.
pub fn dec_quad_from_u32(n: u32) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadFromUInt32(&mut dq, n);
  }
  dq
}

/// Safe binding to *decQuadFromWider* function.
pub fn dec_quad_from_wider(dq1: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadFromWider(&mut dq, dq1, dc);
  }
  dq
}

/// Safe binding to *decQuadGetCoefficient* function.
pub fn dec_quad_get_coefficient(x: &DecQuad, bcd: &[u8; DEC_QUAD_PMAX]) -> c_int {
  unsafe { decQuadGetCoefficient(x, bcd.as_ptr()) }
}

/// Safe binding to *decQuadGetExponent* function.
pub fn dec_quad_get_exponent(x: &DecQuad) -> DecQuad {
  unsafe { decQuadGetExponent(x) }
}

/// Safe binding to *decQuadInvert* function.
pub fn dec_quad_invert(dq1: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadInvert(&mut dq, dq1, dc);
  }
  dq
}

/// Safe binding to *decQuadIsCanonical* function.
pub fn dec_quad_is_canonical(dn: &DecQuad) -> bool {
  unsafe { decQuadIsCanonical(dn) == 1 }
}

/// Safe binding to *decQuadIsFinite* function.
pub fn dec_quad_is_finite(dn: &DecQuad) -> bool {
  unsafe { decQuadIsFinite(dn) == 1 }
}

/// Safe binding to *decQuadIsInfinite* function.
pub fn dec_quad_is_infinite(dn: &DecQuad) -> bool {
  unsafe { decQuadIsInfinite(dn) == 1 }
}

/// Safe binding to *decQuadIsInteger* function.
pub fn dec_quad_is_integer(dn: &DecQuad) -> bool {
  unsafe { decQuadIsInteger(dn) == 1 }
}

/// Safe binding to *decQuadIsLogical* function.
pub fn dec_quad_is_logical(dn: &DecQuad) -> bool {
  unsafe { decQuadIsLogical(dn) == 1 }
}

/// Safe binding to *decQuadIsNaN* function.
pub fn dec_quad_is_nan(dn: &DecQuad) -> bool {
  unsafe { decQuadIsNaN(dn) == 1 }
}

/// Safe binding to *decQuadIsNormal* function.
pub fn dec_quad_is_normal(dn: &DecQuad) -> bool {
  unsafe { decQuadIsNormal(dn) == 1 }
}

/// Safe binding to *decQuadIsNegative* function.
pub fn dec_quad_is_negative(dn: &DecQuad) -> bool {
  unsafe { decQuadIsNegative(dn) == 1 }
}

/// Safe binding to *decQuadIsPositive* function.
pub fn dec_quad_is_positive(dn: &DecQuad) -> bool {
  unsafe { decQuadIsPositive(dn) == 1 }
}

/// Safe binding to *decQuadIsSignaling* function.
pub fn dec_quad_is_signaling(dn: &DecQuad) -> bool {
  unsafe { decQuadIsSignaling(dn) == 1 }
}

/// Safe binding to *decQuadIsSignalling* function.
pub fn dec_quad_is_signalling(dn: &DecQuad) -> bool {
  unsafe { decQuadIsSignalling(dn) == 1 }
}

/// Safe binding to *decQuadIsZero* function.
pub fn dec_quad_is_zero(dn: &DecQuad) -> bool {
  unsafe { decQuadIsZero(dn) == 1 }
}

/// Safe binding to *decQuadMinus* function.
pub fn dec_quad_minus(dn: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadMinus(&mut dq, dn, dc);
  }
  dq
}

/// Safe binding to *decQuadMultiply* function.
pub fn dec_quad_multiply(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadMultiply(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadPlus* function.
pub fn dec_quad_plus(dq1: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadPlus(&mut dq, dq1, dc);
  }
  dq
}

/// Safe binding to *decQuadQuantize* function.
pub fn dec_quad_quantize(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadQuantize(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadReduce* function.
pub fn dec_quad_reduce(dq1: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadReduce(&mut dq, dq1, dc);
  }
  dq
}

/// Safe binding to *decQuadRemainder* function.
pub fn dec_quad_remainder(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadRemainder(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadScaleB* function.
pub fn dec_quad_scale_b(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadScaleB(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadSubtract* function.
pub fn dec_quad_subtract(dq1: &DecQuad, dq2: &DecQuad, dc: &mut DecContext) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadSubtract(&mut dq, dq1, dq2, dc);
  }
  dq
}

/// Safe binding to *decQuadToInt32* function.
pub fn dec_quad_to_int32(dq: &DecQuad, dc: &mut DecContext, rounding: u32) -> i32 {
  unsafe { decQuadToInt32(dq, dc, rounding) }
}

/// Safe binding to *decQuadToIntegralValue* function.
pub fn dec_quad_to_integral_value(dq1: &DecQuad, dc: &mut DecContext, rounding: u32) -> DecQuad {
  let mut dq = DEC_QUAD_ZERO;
  unsafe {
    decQuadToIntegralValue(&mut dq, dq1, dc, rounding);
  }
  dq
}

/// Safe binding to *decQuadToString* function.
pub fn dec_quad_to_string(dq1: &DecQuad) -> String {
  unsafe {
    let mut buf = DEC_QUAD_STRING_BUFFER;
    decQuadToString(dq1, buf.as_mut_ptr() as *mut c_char);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
      .to_string_lossy()
      .into_owned()
  }
}

/// Safe binding to *decQuadToUInt32* function.
pub fn dec_quad_to_uint32(dq: &DecQuad, dc: &mut DecContext, rounding: u32) -> u32 {
  unsafe { decQuadToUInt32(dq, dc, rounding) }
}

/// Safe binding to *decQuadZero* function.
pub fn dec_quad_zero(dq1: &mut DecQuad) {
  unsafe {
    decQuadZero(dq1);
  }
}
