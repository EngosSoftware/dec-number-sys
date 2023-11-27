//! [DecQuad] smoke tests.
//!
//! The purpose of smoke tests is to verify if bindings compile properly.

use dec_number_sys::*;

macro_rules! c {
  () => {
    &mut dec_context_128()
  };
}

macro_rules! n {
  ($s:expr) => {
    dec_quad_from_string(stringify!($s), c!())
  };
}

macro_rules! s {
  ($v:expr) => {
    dec_quad_to_string(&$v)
  };
}

#[test]
fn test_dec_quad_abs() {
  assert_eq!("5.751", s!(dec_quad_abs(&n!(-5.751), c!())));
}

#[test]
fn test_dec_quad_add() {
  assert_eq!("9.05", s!(dec_quad_add(&n!(5.75), &n!(3.3), c!())));
}

#[test]
fn test_dec_quad_and() {
  assert_eq!("1", s!(dec_quad_and(&n!(11), &n!(1), c!())));
  assert_eq!("11", s!(dec_quad_and(&n!(111), &n!(11), c!())));
  assert_eq!("101", s!(dec_quad_and(&n!(10101), &n!(1101), c!())));
  assert_eq!("1000", s!(dec_quad_and(&n!(1111), &n!(11000), c!())));
}

#[test]
fn test_dec_quad_canonical() {
  assert_eq!("1", s!(dec_quad_canonical(&n!(1))));
  assert_eq!("-10", s!(dec_quad_canonical(&n!(-10))));
  assert_eq!("10.25", s!(dec_quad_canonical(&n!(10.25))));
}

#[test]
fn test_dec_quad_compare() {
  assert_eq!("0", s!(dec_quad_compare(&n!(1), &n!(1), c!())));
  assert_eq!("1", s!(dec_quad_compare(&n!(2), &n!(1), c!())));
  assert_eq!("-1", s!(dec_quad_compare(&n!(1), &n!(2), c!())));
  assert_eq!("NaN", s!(dec_quad_compare(&n!(1), &n!(NaN), c!())));
  assert_eq!("NaN", s!(dec_quad_compare(&n!(NaN), &n!(1), c!())));
}

#[test]
fn test_dec_quad_compare_signal() {
  assert_eq!("0", s!(dec_quad_compare_signal(&n!(1), &n!(1), c!())));
  assert_eq!("1", s!(dec_quad_compare_signal(&n!(2), &n!(1), c!())));
  assert_eq!("-1", s!(dec_quad_compare_signal(&n!(1), &n!(2), c!())));
  assert_eq!("NaN", s!(dec_quad_compare_signal(&n!(1), &n!(NaN), c!())));
  assert_eq!("NaN", s!(dec_quad_compare_signal(&n!(NaN), &n!(1), c!())));
  assert_eq!("NaN", s!(dec_quad_compare_signal(&n!(NaN), &n!(NaN), c!())));
}

#[test]
fn test_dec_quad_compare_total() {
  assert_eq!("0", s!(dec_quad_compare_total(&n!(1), &n!(1))));
  assert_eq!("1", s!(dec_quad_compare_total(&n!(2), &n!(1))));
  assert_eq!("-1", s!(dec_quad_compare_total(&n!(1), &n!(2))));
  assert_eq!("-1", s!(dec_quad_compare_total(&n!(1), &n!(NaN))));
  assert_eq!("1", s!(dec_quad_compare_total(&n!(NaN), &n!(1))));
  assert_eq!("0", s!(dec_quad_compare_total(&n!(NaN), &n!(NaN))));
}

#[test]
fn test_dec_quad_compare_total_mag() {
  assert_eq!("0", s!(dec_quad_compare_total_mag(&n!(1), &n!(1))));
  assert_eq!("1", s!(dec_quad_compare_total_mag(&n!(2), &n!(1))));
  assert_eq!("-1", s!(dec_quad_compare_total_mag(&n!(1), &n!(2))));
  assert_eq!("-1", s!(dec_quad_compare_total_mag(&n!(1), &n!(NaN))));
  assert_eq!("1", s!(dec_quad_compare_total_mag(&n!(NaN), &n!(1))));
  assert_eq!("0", s!(dec_quad_compare_total_mag(&n!(NaN), &n!(NaN))));
}

#[test]
fn test_dec_quad_copy() {
  assert_eq!("1", s!(dec_quad_copy(&n!(1))));
  assert_eq!("-10", s!(dec_quad_copy(&n!(-10))));
  assert_eq!("10.25", s!(dec_quad_copy(&n!(10.25))));
}

#[test]
fn test_dec_quad_copy_abs() {
  assert_eq!("1", s!(dec_quad_copy_abs(&n!(1))));
  assert_eq!("10", s!(dec_quad_copy_abs(&n!(-10))));
  assert_eq!("10.25", s!(dec_quad_copy_abs(&n!(10.25))));
}

#[test]
fn test_dec_quad_copy_negate() {
  assert_eq!("-1", s!(dec_quad_copy_negate(&n!(1))));
  assert_eq!("10", s!(dec_quad_copy_negate(&n!(-10))));
  assert_eq!("-10.25", s!(dec_quad_copy_negate(&n!(10.25))));
}

#[test]
fn test_dec_quad_copy_sign() {
  assert_eq!("1", s!(dec_quad_copy_sign(&n!(1), &n!(1))));
  assert_eq!("10", s!(dec_quad_copy_sign(&n!(-10), &n!(1))));
  assert_eq!("-10.25", s!(dec_quad_copy_sign(&n!(10.25), &n!(-1))));
}

#[test]
fn test_dec_quad_constants() {
  // zero
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]",
    format!("{:?}", n!(0))
  );
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00]",
    format!("{:?}", DEC_QUAD_ZERO)
  );
  // one
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]",
    format!("{:?}", n!(1))
  );
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 01]",
    format!("{:?}", DEC_QUAD_ONE)
  );
  // two
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 02]",
    format!("{:?}", n!(2))
  );
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 02]",
    format!("{:?}", DEC_QUAD_TWO)
  );
  // ten
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]",
    format!("{:?}", n!(10))
  );
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 10]",
    format!("{:?}", DEC_QUAD_TEN)
  );
  // hundred
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]",
    format!("{:?}", n!(100))
  );
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 00 80]",
    format!("{:?}", DEC_QUAD_HUNDRED)
  );
  // thousand
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]",
    format!("{:?}", n!(1000))
  );
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 00 04 00]",
    format!("{:?}", DEC_QUAD_THOUSAND)
  );
  // million
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]",
    format!("{:?}", n!(1000000))
  );
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 00 10 00 00]",
    format!("{:?}", DEC_QUAD_MILLION)
  );
  // billion
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 40 00 00 00]",
    format!("{:?}", n!(1000000000))
  );
  assert_eq!(
    "[22 08 00 00 00 00 00 00 00 00 00 00 40 00 00 00]",
    format!("{:?}", DEC_QUAD_BILLION)
  );
}

#[test]
#[allow(clippy::clone_on_copy)]
fn test_dec_quad_copy_clone() {
  let n1 = n!(1.2345);
  let n2 = n1;
  let n3 = n1.clone();
  assert_eq!("1.2345", s!(n2));
  assert_eq!("1.2345", s!(n3));
}

#[test]
fn test_dec_quad_digits() {
  assert_eq!(1, dec_quad_digits(&n!(1)));
  assert_eq!(2, dec_quad_digits(&n!(10)));
  assert_eq!(2, dec_quad_digits(&n!(0.75)));
}

#[test]
fn test_dec_quad_divide() {
  assert_eq!("0.25", s!(dec_quad_divide(&n!(1), &n!(4), c!())));
}

#[test]
fn test_dec_quad_divide_fma() {
  assert_eq!("2", s!(dec_quad_fma(&n!(1), &n!(1), &n!(1), c!())));
  assert_eq!("24", s!(dec_quad_fma(&n!(5), &n!(4), &n!(4), c!())));
  assert_eq!("0.00", s!(dec_quad_fma(&n!(2.5), &n!(2.5), &n!(-6.25), c!())));
}

#[test]
fn test_dec_quad_divide_integer() {
  assert_eq!("0", s!(dec_quad_divide_integer(&n!(1), &n!(4), c!())));
  assert_eq!("1", s!(dec_quad_divide_integer(&n!(5), &n!(4), c!())));
}

#[test]
fn test_dec_quad_from_bcd() {
  assert_eq!(
    "2366920938463463374607431768211455",
    s!(dec_quad_from_bcd(&bcd_quad(u128::MAX), 0, false))
  );
  assert_eq!(
    "18446744073709551615",
    s!(dec_quad_from_bcd(&bcd_quad(u64::MAX.into()), 0, false))
  );
  assert_eq!(
    "-18446744073709551615",
    s!(dec_quad_from_bcd(&bcd_quad(u64::MAX.into()), 0, true))
  );
  assert_eq!(
    "9223372036854775807",
    s!(dec_quad_from_bcd(&bcd_quad(i64::MAX.unsigned_abs().into()), 0, false))
  );
  assert_eq!(
    "-9223372036854775808",
    s!(dec_quad_from_bcd(&bcd_quad(i64::MIN.unsigned_abs().into()), 0, true))
  );
  assert_eq!(
    "1844674407.3709551615",
    s!(dec_quad_from_bcd(&bcd_quad(u64::MAX.into()), -10, false))
  );
}

#[test]
fn test_dec_quad_from_i32() {
  assert_eq!("-12", s!(dec_quad_from_i32(-12)));
}

// #[test]
// fn test_dec_quad_from_number() {
//   let dn = dec_number_from_u32(32);
//   assert_eq!("32", s!(dec_quad_from_number(&dn, c!())));
// }

// #[test]
//
// fn test_dec_quad_from_packed() {
//   assert_eq!("2366920938463463374607431768211455", s!(dec_quad_from_packed(&bcd_quad(u128::MAX), 0, false)));
//   assert_eq!("18446744073709551615", s!(dec_quad_from_packed(&bcd_quad(u64::MAX.into()), 0, false)));
//   assert_eq!("-18446744073709551615", s!(dec_quad_from_packed(&bcd_quad(u64::MAX.into()), 0, true)));
//   assert_eq!("9223372036854775807", s!(dec_quad_from_packed(&bcd_quad(i64::MAX.unsigned_abs().into()), 0, false)));
//   assert_eq!("-9223372036854775808", s!(dec_quadec_quad_from_packedd_from_bcd(&bcd_quad(i64::MIN.unsigned_abs().into()), 0, true)));
//   assert_eq!("1844674407.3709551615", s!(dec_quad_from_packed(&bcd_quad(u64::MAX.into()), -10, false)));
// }

#[test]
fn test_dec_quad_from_u32() {
  assert_eq!("12", s!(dec_quad_from_u32(12)));
}

// #[test]
// fn test_dec_quad_from_wider() {
//   assert_eq!("12", s!(dec_quad_from_wider(&n!(10), c!())));
// }

#[test]
fn test_dec_quad_get_coefficient() {
  assert_eq!(0, dec_quad_get_coefficient(&n!(1), &mut bcd_quad(u128::MAX)));
  assert_eq!(-2147483648, dec_quad_get_coefficient(&n!(-1), &mut bcd_quad(u128::MAX)));
}

#[test]
fn test_dec_quad_get_exponent() {
  assert_eq!("4.0000000000000000000000E-6154", s!(dec_quad_get_exponent(&n!(1))));
  assert_eq!("4.0000000000000000000000E-6154", s!(dec_quad_get_exponent(&n!(10000))));
}

#[test]
fn test_dec_quad_invert() {
  assert_eq!("1111111111111111111111111111111111", s!(dec_quad_invert(&n!(0), c!())));
  assert_eq!("1111111111111111111111111111111110", s!(dec_quad_invert(&n!(1), c!())));
  assert_eq!("1111111111111111111111111111111100", s!(dec_quad_invert(&n!(11), c!())));
}

#[test]
fn test_dec_quad_is_canonical() {
  assert!(dec_quad_is_canonical(&n!(1)));
  assert!(dec_quad_is_canonical(&n!(0)));
}

#[test]
fn test_dec_quad_is_finite() {
  assert!(dec_quad_is_finite(&n!(1)));
  assert!(!dec_quad_is_finite(&n!(NaN)));
}

#[test]
fn test_dec_quad_is_infinite() {
  assert!(!dec_quad_is_infinite(&n!(1)));
  assert!(!dec_quad_is_infinite(&n!(0)));
}

#[test]
fn test_dec_quad_is_integer() {
  assert!(dec_quad_is_integer(&n!(1)));
  assert!(!dec_quad_is_integer(&n!(1.1)));
}

#[test]
fn test_dec_quad_is_logical() {
  assert!(dec_quad_is_logical(&n!(1)));
  assert!(!dec_quad_is_logical(&n!(1.1)));
}

#[test]
fn test_dec_quad_is_nan() {
  assert!(!dec_quad_is_nan(&n!(1)));
  assert!(dec_quad_is_nan(&n!(NaN)));
}

#[test]
fn test_dec_quad_is_normal() {
  assert!(dec_quad_is_normal(&n!(1)));
  assert!(!dec_quad_is_normal(&n!(NaN)));
}

#[test]
fn test_dec_quad_is_negative() {
  assert!(dec_quad_is_negative(&n!(-1)));
  assert!(!dec_quad_is_negative(&n!(1)));
  assert!(!dec_quad_is_negative(&n!(NaN)));
}

#[test]
fn test_dec_quad_is_positive() {
  assert!(dec_quad_is_positive(&n!(1)));
  assert!(!dec_quad_is_positive(&n!(-1)));
  assert!(!dec_quad_is_positive(&n!(NaN)));
}

#[test]
fn test_dec_quad_is_signaling() {
  assert!(!dec_quad_is_signaling(&n!(1)));
  assert!(!dec_quad_is_signaling(&n!(0)));
  assert!(dec_quad_is_signaling(&n!(sNaN)));
}

#[test]
fn test_dec_quad_is_signalling() {
  assert!(!dec_quad_is_signalling(&n!(1)));
  assert!(!dec_quad_is_signalling(&n!(0)));
  assert!(dec_quad_is_signalling(&n!(sNaN)));
}

#[test]
fn test_dec_quad_is_zero() {
  assert!(dec_quad_is_zero(&n!(0)));
  assert!(!dec_quad_is_zero(&n!(1)));
  assert!(!dec_quad_is_zero(&n!(NaN)));
}

#[test]
fn test_dec_quad_minus() {
  assert_eq!("0", s!(dec_quad_minus(&n!(-0), c!())));
  assert_eq!("0", s!(dec_quad_minus(&n!(0), c!())));
  assert_eq!("1.1", s!(dec_quad_minus(&n!(-1.1), c!())));
  assert_eq!("-1.1", s!(dec_quad_minus(&n!(1.1), c!())));
}

#[test]
fn test_dec_quad_multiply() {
  assert_eq!("4.4", s!(dec_quad_multiply(&n!(1.1), &n!(4), c!())));
}

#[test]
fn test_dec_quad_plus() {
  assert_eq!("0", s!(dec_quad_plus(&n!(-0), c!())));
  assert_eq!("0", s!(dec_quad_plus(&n!(0), c!())));
  assert_eq!("-1.1", s!(dec_quad_plus(&n!(-1.1), c!())));
  assert_eq!("1.1", s!(dec_quad_plus(&n!(1.1), c!())));
}

#[test]
fn test_dec_quad_quantize() {
  assert_eq!("123.5", s!(dec_quad_quantize(&n!(123.456), &n!(1.0), c!())));
}

#[test]
fn test_dec_quad_reduce() {
  assert_eq!("1.2345678E+11", s!(dec_quad_reduce(&n!(12345678E+4), c!())));
}

#[test]
fn test_dec_quad_remainder() {
  assert_eq!("1", s!(dec_quad_remainder(&n!(10), &n!(3), c!())));
}

#[test]
fn test_dec_quad_scale_b() {
  assert_eq!("1234.5678", s!(dec_quad_scale_b(&n!(12345678), &n!(-4), c!())));
}

#[test]
fn test_dec_quad_subtract() {
  assert_eq!("1.000", s!(dec_quad_subtract(&n!(1.123), &n!(0.123), c!())));
}

#[test]
fn test_dec_quad_to_int32() {
  assert_eq!(0, dec_quad_to_int32(&n!(0.999), c!(), DEC_ROUND_DOWN));
  assert_eq!(0, dec_quad_to_int32(&n!(-0.999), c!(), DEC_ROUND_DOWN));
  assert_eq!(1, dec_quad_to_int32(&n!(0.5), c!(), DEC_ROUND_HALF_UP));
  assert_eq!(-1, dec_quad_to_int32(&n!(-0.5), c!(), DEC_ROUND_HALF_UP));
  assert_eq!(0, dec_quad_to_int32(&n!(0.4), c!(), DEC_ROUND_HALF_UP));
}

#[test]
fn test_dec_quad_to_integral_value() {
  assert_eq!(
    "-0",
    s!(dec_quad_to_integral_value(&n!(-0.75), c!(), DEC_ROUND_CEILING))
  );
}

#[test]
fn test_dec_quad_to_uint32() {
  assert_eq!(0, dec_quad_to_uint32(&n!(0.999), c!(), DEC_ROUND_DOWN));
  assert_eq!(1, dec_quad_to_uint32(&n!(0.5), c!(), DEC_ROUND_HALF_UP));
  assert_eq!(0, dec_quad_to_uint32(&n!(0.4), c!(), DEC_ROUND_HALF_UP));
  assert_eq!(4294967295, dec_quad_to_uint32(&n!(4294967295), c!(), DEC_ROUND_DOWN));
  let ctx = &mut dec_context_128();
  assert_eq!(0, dec_quad_to_uint32(&n!(4294967296), ctx, DEC_ROUND_DOWN));
  assert_ne!(0, dec_context_get_status(ctx));
  assert_eq!("Invalid operation", dec_context_status_to_string(ctx));
}

#[test]
fn test_dec_quad_zero() {
  let mut n = n!(-0.75);
  dec_quad_zero(&mut n);
  assert_eq!("0", s!(n));
}
