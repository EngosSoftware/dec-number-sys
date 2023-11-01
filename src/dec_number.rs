//! Safe bindings for arbitrary precision decimal.

use crate::dec_context::*;
use crate::dec_number_c::*;
use libc::c_char;
use std::ffi::{CStr, CString};

/// Sign mask: 1 = negative, 0 = positive or zero.
pub const DEC_NEG: u8 = 0x80;

/// Infinity mask: 1 = Infinity.
pub const DEC_INF: u8 = 0x40;

/// Not-a-Number mask: 1 = NaN.
pub const DEC_NAN: u8 = 0x20;

/// Signalling Not-a-Number mask: 1 = sNaN.
pub const DEC_SNAN: u8 = 0x10;

/// Special value mask: 1 = special (Infinity, NaN or sNaN).
pub const DEC_SPECIAL: u8 = DEC_INF | DEC_NAN | DEC_SNAN;

/// [DecNumber] value 0 (zero).
pub const DEC_NUMBER_ZERO: DecNumber = DecNumber {
  digits: 1,
  exponent: 0,
  bits: 0,
  lsu: [0; 12],
};

/// [DecNumber] value 1 (one).
pub const DEC_NUMBER_ONE: DecNumber = DecNumber {
  digits: 1,
  exponent: 0,
  bits: 0,
  lsu: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// [DecNumber] value 2 (two).
pub const DEC_NUMBER_TWO: DecNumber = DecNumber {
  digits: 1,
  exponent: 0,
  bits: 0,
  lsu: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// [DecNumber] value 10 (ten).
pub const DEC_NUMBER_TEN: DecNumber = DecNumber {
  digits: 2,
  exponent: 0,
  bits: 0,
  lsu: [10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// [DecNumber] value 100 (hundred).
pub const DEC_NUMBER_HUNDRED: DecNumber = DecNumber {
  digits: 3,
  exponent: 0,
  bits: 0,
  lsu: [100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// [DecNumber] value 1000 (thousand).
pub const DEC_NUMBER_THOUSAND: DecNumber = DecNumber {
  digits: 4,
  exponent: 0,
  bits: 0,
  lsu: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// [DecNumber] value 1000000 (million).
pub const DEC_NUMBER_MILLION: DecNumber = DecNumber {
  digits: 7,
  exponent: 0,
  bits: 0,
  lsu: [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// [DecNumber] value 1000000000 (billion).
pub const DEC_NUMBER_BILLION: DecNumber = DecNumber {
  digits: 10,
  exponent: 0,
  bits: 0,
  lsu: [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
};

/// Arbitrary precision decimal number.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DecNumber {
  digits: i32,
  exponent: i32,
  bits: u8,
  lsu: [u16; 12],
}

impl Default for DecNumber {
  /// Default value for [DecNumber] is zero.
  fn default() -> Self {
    DEC_NUMBER_ZERO
  }
}

/// Safe binding to *decNumberAdd* function.
pub fn dec_number_add(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberAdd(&mut dn, dn1, dn2, dc);
  }
  dn
}

/// Safe binding to *decNumberCompare* function.
pub fn dec_number_compare(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberCompare(&mut dn, dn1, dn2, dc);
  }
  dn
}

/// Safe binding to *decNumberDivide* function.
pub fn dec_number_divide(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberDivide(&mut dn, dn1, dn2, dc);
  }
  dn
}

/// Safe binding to *decNumberExp* function.
pub fn dec_number_exp(dn1: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberExp(&mut dn, dn1, dc);
  }
  dn
}

/// Safe binding to *decNumberFromInt32* function.
pub fn dec_number_from_i32(n: i32) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberFromInt32(&mut dn, n);
  }
  dn
}

/// Safe binding to *decNumberFromString* function.
pub fn dec_number_from_string(s: &str, dc: &mut DecContext) -> DecNumber {
  let c_s = CString::new(s).unwrap();
  let mut value = DEC_NUMBER_ZERO;
  unsafe {
    decNumberFromString(&mut value, c_s.as_ptr(), dc);
  }
  value
}

/// Safe binding to *decNumberFromUInt32* function.
pub fn dec_number_from_u32(n: u32) -> DecNumber {
  let mut result = DEC_NUMBER_ZERO;
  unsafe {
    decNumberFromUInt32(&mut result, n);
  }
  result
}

/// Safe binding to *decNumberIsNegative* function.
///
/// This function was replaced by macro and removed from public API.
/// This implementation is the Rust version of original library macro.
pub fn dec_number_is_negative(dn1: &DecNumber) -> bool {
  dn1.bits & DEC_NEG != 0
}

/// Safe binding to *decNumberIsZero* function.
///
/// This function was replaced by macro and removed from public API.
/// This implementation is the Rust version of original library macro.
pub fn dec_number_is_zero(dn1: &DecNumber) -> bool {
  dn1.lsu[0] == 0 && dn1.digits == 1 && (dn1.bits & DEC_SPECIAL == 0)
}

/// Safe binding to *decNumberLn* function.
pub fn dec_number_ln(dn1: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberLn(&mut dn, dn1, dc);
  }
  dn
}

/// Safe binding to *decNumberMinus* function.
pub fn dec_number_minus(dn1: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberMinus(&mut dn, dn1, dc);
  }
  dn
}

/// Safe binding to *decNumberMultiply* function.
pub fn dec_number_multiply(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberMultiply(&mut dn, dn1, dn2, dc);
  }
  dn
}

/// Safe binding to *decNumberPlus* function.
pub fn dec_number_plus(dn1: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberPlus(&mut dn, dn1, dc);
  }
  dn
}

/// Safe binding to *decNumberPower* function.
pub fn dec_number_power(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberPower(&mut dn, dn1, dn2, dc);
  }
  dn
}

/// Safe binding to *decNumberQuantize* function.
pub fn dec_number_quantize(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberQuantize(&mut dn, dn1, dn2, dc);
  }
  dn
}

/// Safe binding to *decNumberReduce* function.
pub fn dec_number_reduce(dn1: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberReduce(&mut dn, dn1, dc);
  }
  dn
}

/// Safe binding to *decNumberRescale* function.
pub fn dec_number_rescale(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberRescale(&mut dn, dn1, dn2, dc);
  }
  dn
}

/// Safe binding to *decNumberScaleB* function.
pub fn dec_number_scale_b(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberScaleB(&mut dn, dn1, dn2, dc);
  }
  dn
}

/// Safe binding to *decNumberSquareRoot* function.
pub fn dec_number_square_root(dn1: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberSquareRoot(&mut dn, dn1, dc);
  }
  dn
}

/// Safe binding to *decNumberSubtract* function.
pub fn dec_number_subtract(dn1: &DecNumber, dn2: &DecNumber, dc: &mut DecContext) -> DecNumber {
  let mut dn = DEC_NUMBER_ZERO;
  unsafe {
    decNumberSubtract(&mut dn, dn1, dn2, dc);
  }
  dn
}

/// Safe binding to *decNumberToString* function.
pub fn dec_number_to_string(dn1: &DecNumber) -> String {
  unsafe {
    let mut buf = Vec::<char>::with_capacity((dn1.digits + 14) as usize);
    decNumberToString(dn1, buf.as_mut_ptr() as *mut c_char);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
      .to_string_lossy()
      .into_owned()
  }
}

/// Safe binding to *decNumberZero* function.
pub fn dec_number_zero(dn1: &mut DecNumber) {
  unsafe {
    decNumberZero(dn1);
  }
}
