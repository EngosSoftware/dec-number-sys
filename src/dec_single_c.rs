//! Unsafe bindings for 32-bit decimal.

use crate::{DecContext, DecDouble, DecSingle};
use libc::c_char;

extern "C" {
  /// Unsafe binding to *decSingleFromString* function.
  pub fn decSingleFromString(ds: *mut DecSingle, s: *const c_char, dc: *mut DecContext) -> *mut DecSingle;
  /// Unsafe binding to *decSingleFromWider* function.
  pub fn decSingleFromWider(ds: *mut DecSingle, ds1: *const DecDouble, dc: *mut DecContext) -> *mut DecSingle;
  /// Unsafe binding to *decSingleToString* function.
  pub fn decSingleToString(ds: *const DecSingle, s: *mut c_char) -> *mut c_char;
  /// Unsafe binding to *decSingleToWider* function.
  pub fn decSingleToWider(ds: *const DecSingle, ds1: *mut DecDouble, dc: *mut DecContext) -> *mut DecDouble;
  /// Unsafe binding to *decSingleZero* function.
  pub fn decSingleZero(ds: *mut DecSingle);
}

/*
  pub fn decSingleAbs() {}
  pub fn decSingleAdd() {}
  pub fn decSingleAnd() {}
  pub fn decSingleCanonical() {}
  pub fn decSingleClass() {}
  pub fn decSingleClassString() {}
  pub fn decSingleCompare() {}
  pub fn decSingleCompareSignal() {}
  pub fn decSingleCompareTotal() {}
  pub fn decSingleCompareTotalMag() {}
  pub fn decSingleCopy() {}
  pub fn decSingleCopyAbs() {}
  pub fn decSingleCopyNegate() {}
  pub fn decSingleCopySign() {}
  pub fn decSingleDigits() {}
  pub fn decSingleDivide() {}
  pub fn decSingleDivideInteger() {}
  pub fn decSingleFMA() {}
  pub fn decSingleFromBCD() {}
  pub fn decSingleFromInt32() {}
  pub fn decSingleFromNumber() {}
  pub fn decSingleFromPacked() {}
  pub fn decSingleFromPackedChecked() {}
  pub fn decSingleFromString() {}
  pub fn decSingleFromUInt32() {}
  pub fn decSingleFromWider() {}
  pub fn decSingleGetCoefficient() {}
  pub fn decSingleGetExponent() {}
  pub fn decSingleInvert() {}
  pub fn decSingleIsCanonical() {}
  pub fn decSingleIsFinite() {}
  pub fn decSingleIsInfinite() {}
  pub fn decSingleIsInteger() {}
  pub fn decSingleIsLogical() {}
  pub fn decSingleIsNaN() {}
  pub fn decSingleIsNegative() {}
  pub fn decSingleIsNormal() {}
  pub fn decSingleIsPositive() {}
  pub fn decSingleIsSignaling() {}
  pub fn decSingleIsSignalling() {}
  pub fn decSingleIsSigned() {}
  pub fn decSingleIsSubnormal() {}
  pub fn decSingleIsZero() {}
  pub fn decSingleLogB() {}
  pub fn decSingleMax() {}
  pub fn decSingleMaxMag() {}
  pub fn decSingleMin() {}
  pub fn decSingleMinMag() {}
  pub fn decSingleMinus() {}
  pub fn decSingleMultiply() {}
  pub fn decSingleNextMinus() {}
  pub fn decSingleNextPlus() {}
  pub fn decSingleNextToward() {}
  pub fn decSingleOr() {}
  pub fn decSinglePlus() {}
  pub fn decSingleQuantize() {}
  pub fn decSingleRadix() {}
  pub fn decSingleReduce() {}
  pub fn decSingleRemainder() {}
  pub fn decSingleRemainderNear() {}
  pub fn decSingleRotate() {}
  pub fn decSingleSameQuantum() {}
  pub fn decSingleScaleB() {}
  pub fn decSingleSetCoefficient() {}
  pub fn decSingleSetExponent() {}
  pub fn decSingleShift() {}
  pub fn decSingleShow() {}
  pub fn decSingleSubtract() {}
  pub fn decSingleToBCD() {}
  pub fn decSingleToEngString() {}
  pub fn decSingleToInt32() {}
  pub fn decSingleToInt32Exact() {}
  pub fn decSingleToIntegralExact() {}
  pub fn decSingleToIntegralValue() {}
  pub fn decSingleToNumber() {}
  pub fn decSingleToPacked() {}
  pub fn decSingleToString() {}
  pub fn decSingleToUInt32() {}
  pub fn decSingleToUInt32Exact() {}
  pub fn decSingleToWider() {}
  pub fn decSingleVersion() {}
  pub fn decSingleXor() {}
  pub fn decSingleToUInt32() {}
  pub fn decSingleToUInt32Exact() {}
  pub fn decSingleToWider() {}
  pub fn decSingleVersion() {}
  pub fn decSingleXor() {}
  pub fn decSingleZero() {}
*/
