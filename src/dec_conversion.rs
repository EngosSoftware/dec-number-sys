//! Safe bindings for decimal conversion functions.

use crate::dec_conversion_c::*;
use crate::dec_double::DecDouble;
use crate::dec_number::DecNumber;
use crate::dec_quad::DecQuad;
use crate::dec_single::DecSingle;
use crate::DecContext;

/// Safe binding to *decimal32ToNumber* function.
pub fn decimal32_to_number<'a>(ds: &'a DecSingle, dn: &'a mut DecNumber) -> &'a DecNumber {
  unsafe {
    decimal32ToNumber(ds, dn);
    dn
  }
}

/// Safe binding to *decimal32FromNumber* function.
pub fn decimal32_from_number(dn: &DecNumber, dc: &mut DecContext) -> DecSingle {
  let mut ds = DecSingle::default();
  unsafe {
    decimal32FromNumber(&mut ds, dn, dc);
  }
  ds
}

/// Safe binding to *decimal64ToNumber* function.
pub fn decimal64_to_number<'a>(dd: &'a DecDouble, dn: &'a mut DecNumber) -> &'a DecNumber {
  unsafe {
    decimal64ToNumber(dd, dn);
    dn
  }
}

/// Safe binding to *decimal64FromNumber* function.
pub fn decimal64_from_number(dn: &DecNumber, dc: &mut DecContext) -> DecDouble {
  let mut dd = DecDouble::default();
  unsafe {
    decimal64FromNumber(&mut dd, dn, dc);
  }
  dd
}

/// Safe binding to *decimal128ToNumber* function.
pub fn decimal128_to_number<'a>(dq: &'a DecQuad, dn: &'a mut DecNumber) -> &'a DecNumber {
  unsafe {
    decimal128ToNumber(dq, dn);
    dn
  }
}

/// Safe binding to *decimal128FromNumber* function.
pub fn decimal128_from_number(dn: &DecNumber, dc: &mut DecContext) -> DecQuad {
  let mut dq = DecQuad::default();
  unsafe {
    decimal128FromNumber(&mut dq, dn, dc);
  }
  dq
}
