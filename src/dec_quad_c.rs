//! Unsafe bindings for 128-bit decimal.

use crate::{DecContext, DecQuad};
use libc::{c_char, c_int, c_uchar, c_uint};

extern "C" {
  /// Unsafe binding to *decQuadAbs* function.
  pub fn decQuadAbs(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadAdd* function.
  pub fn decQuadAdd(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadAnd* function.
  pub fn decQuadAnd(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decDoubleCanonical* function.
  pub fn decQuadCanonical(dq: *mut DecQuad, dq1: *const DecQuad) -> *mut DecQuad;
  /// Unsafe binding to *decQuadCompare* function.
  pub fn decQuadCompare(
    dq: *mut DecQuad,
    dq1: *const DecQuad,
    dq2: *const DecQuad,
    dc: *mut DecContext,
  ) -> *mut DecQuad;
  /// Unsafe binding to *decQuadCompareSignal* function.
  pub fn decQuadCompareSignal(
    dq: *mut DecQuad,
    dq1: *const DecQuad,
    dq2: *const DecQuad,
    dc: *mut DecContext,
  ) -> *mut DecQuad;
  /// Unsafe binding to *decQuadCompareTotal* function.
  pub fn decQuadCompareTotal(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad) -> *mut DecQuad;
  /// Unsafe binding to *decQuadCompareTotalMag* function.
  pub fn decQuadCompareTotalMag(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad) -> *mut DecQuad;
  /// Unsafe binding to *decQuadCopy* function.
  pub fn decQuadCopy(dq: *mut DecQuad, dq1: *const DecQuad) -> *mut DecQuad;
  /// Unsafe binding to *decQuadCopyAbs* function.
  pub fn decQuadCopyAbs(dq: *mut DecQuad, dq1: *const DecQuad) -> *mut DecQuad;
  /// Unsafe binding to *decQuadCopyNegate* function.
  pub fn decQuadCopyNegate(dq: *mut DecQuad, dq1: *const DecQuad) -> *mut DecQuad;
  /// Unsafe binding to *decQuadCopySign* function.
  pub fn decQuadCopySign(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad) -> *mut DecQuad;
  /// Unsafe binding to *decQuadDigits* function.
  pub fn decQuadDigits(dq1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadDivide* function.
  pub fn decQuadDivide(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext)
    -> *mut DecQuad;
  /// Unsafe binding to *decQuadDivideInteger* function.
  pub fn decQuadDivideInteger(
    dq: *mut DecQuad,
    dq1: *const DecQuad,
    dq2: *const DecQuad,
    dc: *mut DecContext,
  ) -> *mut DecQuad;
  /// Unsafe binding to *decQuadFMA* function.
  pub fn decQuadFMA(
    dq: *mut DecQuad,
    dq1: *const DecQuad,
    dq2: *const DecQuad,
    dq3: *const DecQuad,
    dc: *mut DecContext,
  ) -> *mut DecQuad;
  /// Unsafe binding to *decQuadFromBCD* function.
  pub fn decQuadFromBCD(dq: *mut DecQuad, exp: c_int, bcd: *const c_uchar, sign: c_int) -> *mut DecQuad;
  /// Unsafe binding to *decQuadFromInt32* function.
  pub fn decQuadFromInt32(dq: *mut DecQuad, n: c_int) -> *mut DecQuad;
  //  /// Unsafe binding to *decQuadFromNumber* function.
  //  pub fn decQuadFromNumber(dq: *mut DecQuad, dn: *const DecNumber, dc: *mut DecContext) -> *mut DecQuad;
  //   /// Unsafe binding to *decQuadFromPacked* function.
  //   pub fn decQuadFromPacked(dq: *mut DecQuad, exp: c_int, pack: *const c_uchar) -> *mut DecQuad;
  /// Unsafe binding to *decQuadFromString* function.
  pub fn decQuadFromString(dq: *mut DecQuad, s: *const c_char, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadFromUInt32* function.
  pub fn decQuadFromUInt32(dq: *mut DecQuad, n: c_uint) -> *mut DecQuad;
  /// Unsafe binding to *decQuadFromWider* function.
  pub fn decQuadFromWider(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadGetCoefficient* function.
  pub fn decQuadGetCoefficient(dq1: *const DecQuad, bcd: *const c_uchar) -> c_int;
  /// Unsafe binding to *decQuadGetExponent* function.
  pub fn decQuadGetExponent(dq1: *const DecQuad) -> DecQuad;
  /// Unsafe binding to *decQuadInvert* function.
  pub fn decQuadInvert(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadIsCanonical* function.
  pub fn decQuadIsCanonical(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsFinite* function.
  pub fn decQuadIsFinite(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsInfinite* function.
  pub fn decQuadIsInfinite(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsInteger* function.
  pub fn decQuadIsInteger(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsLogical* function.
  pub fn decQuadIsLogical(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsNaN* function.
  pub fn decQuadIsNaN(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsNegative* function.
  pub fn decQuadIsNegative(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsNormal* function.
  pub fn decQuadIsNormal(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsPositive* function.
  pub fn decQuadIsPositive(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsSignaling* function.
  pub fn decQuadIsSignaling(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsSignalling* function.
  pub fn decQuadIsSignalling(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadIsZero* function.
  pub fn decQuadIsZero(arg1: *const DecQuad) -> c_uint;
  /// Unsafe binding to *decQuadMinus* function.
  pub fn decQuadMinus(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadMultiply* function.
  pub fn decQuadMultiply(
    dq: *mut DecQuad,
    dq1: *const DecQuad,
    dq2: *const DecQuad,
    dc: *mut DecContext,
  ) -> *mut DecQuad;
  /// Unsafe binding to *decQuadPlus* function.
  pub fn decQuadPlus(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadQuantize* function.
  pub fn decQuadQuantize(
    dq: *mut DecQuad,
    dq1: *const DecQuad,
    dq2: *const DecQuad,
    dc: *mut DecContext,
  ) -> *mut DecQuad;
  /// Unsafe binding to *decQuadReduce* function.
  pub fn decQuadReduce(dq: *mut DecQuad, dq1: *const DecQuad, dc: *mut DecContext) -> *mut DecQuad;
  /// Unsafe binding to *decQuadRemainder* function.
  pub fn decQuadRemainder(
    dq: *mut DecQuad,
    dq1: *const DecQuad,
    dq2: *const DecQuad,
    dc: *mut DecContext,
  ) -> *mut DecQuad;
  /// Unsafe binding to *decQuadScaleB* function.
  pub fn decQuadScaleB(dq: *mut DecQuad, dq1: *const DecQuad, dq2: *const DecQuad, dc: *mut DecContext)
    -> *mut DecQuad;
  /// Unsafe binding to *decQuadSubtract* function.
  pub fn decQuadSubtract(
    dq: *mut DecQuad,
    dq1: *const DecQuad,
    dq2: *const DecQuad,
    dc: *mut DecContext,
  ) -> *mut DecQuad;
  /// Unsafe binding to *decQuadToInt32* function.
  pub fn decQuadToInt32(dq: *const DecQuad, dc: *mut DecContext, rounding: u32) -> c_int;
  /// Unsafe binding to *decQuadToIntegralValue* function.
  pub fn decQuadToIntegralValue(
    dq: *mut DecQuad,
    dq1: *const DecQuad,
    dc: *mut DecContext,
    rounding: u32,
  ) -> *mut DecQuad;
  /// Unsafe binding to *decQuadToString* function.
  pub fn decQuadToString(dq: *const DecQuad, s: *mut c_char) -> *mut c_char;
  /// Unsafe binding to *decQuadToUInt32* function.
  pub fn decQuadToUInt32(dq: *const DecQuad, dc: *mut DecContext, rounding: u32) -> c_uint;
  /// Unsafe binding to *decQuadZero* function.
  pub fn decQuadZero(dq: *mut DecQuad);
}

/*


  pub fn decQuadClass(){}
  pub fn decQuadClassString(){}
  pub fn decQuadFromNumber(){}
  pub fn decQuadFromPacked(){}
  pub fn decQuadFromPackedChecked(){}
  pub fn decQuadFromWider(){}
  pub fn decQuadIsSigned(){}
  pub fn decQuadIsSubnormal(){}
  pub fn decQuadLogB(){}
  pub fn decQuadMax(){}
  pub fn decQuadMaxMag(){}
  pub fn decQuadMin(){}
  pub fn decQuadMinMag(){}
  pub fn decQuadNextMinus(){}
  pub fn decQuadNextPlus(){}
  pub fn decQuadNextToward(){}
  pub fn decQuadOr(){}
  pub fn decQuadRadix(){}
  pub fn decQuadRemainderNear(){}
  pub fn decQuadRotate(){}
  pub fn decQuadSameQuantum(){}
  pub fn decQuadSetCoefficient(){}
  pub fn decQuadSetExponent(){}
  pub fn decQuadShift(){}
  pub fn decQuadShow(){}
  pub fn decQuadToBCD(){}
  pub fn decQuadToEngString(){}
  pub fn decQuadToInt32Exact(){}
  pub fn decQuadToIntegralExact(){}
  pub fn decQuadToNumber(){}
  pub fn decQuadToPacked(){}
  pub fn decQuadToUInt32(){}
  pub fn decQuadToUInt32Exact(){}
  pub fn decQuadToWider(){}
  pub fn decQuadVersion(){}
  pub fn decQuadXor(){}
  pub fn decQuadToUInt32Exact(){}
  pub fn decQuadToWider(){}
  pub fn decQuadVersion(){}
  pub fn decQuadXor(){}
  pub fn decQuadZero(){}
*/
