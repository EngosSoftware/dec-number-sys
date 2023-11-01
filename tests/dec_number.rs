//! [DecNumber] smoke tests.
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
    dec_number_from_string(stringify!($s), c!())
  };
}

macro_rules! s {
  ($v:expr) => {
    dec_number_to_string(&$v)
  };
}

#[test]
fn test_dec_number_add() {
  assert_eq!("1.999", s!(dec_number_add(&n!(1.234), &n!(0.765), c!())));
}

#[test]
fn test_dec_number_compare() {
  assert_eq!("0", s!(dec_number_compare(&n!(1), &n!(1), c!())));
  assert_eq!("1", s!(dec_number_compare(&n!(2), &n!(1), c!())));
  assert_eq!("-1", s!(dec_number_compare(&n!(1), &n!(2), c!())));
  assert_eq!("NaN", s!(dec_number_compare(&n!(1), &n!(NaN), c!())));
  assert_eq!("NaN", s!(dec_number_compare(&n!(NaN), &n!(1), c!())));
}

#[test]
#[rustfmt::skip]
fn test_dec_quad_constants() {
  // zero
  assert_eq!("DecNumber { digits: 1, exponent: 0, bits: 0, lsu: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", n!(0)));
  assert_eq!("DecNumber { digits: 1, exponent: 0, bits: 0, lsu: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", DEC_NUMBER_ZERO));
  // one
  assert_eq!("DecNumber { digits: 1, exponent: 0, bits: 0, lsu: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", n!(1)));
  assert_eq!("DecNumber { digits: 1, exponent: 0, bits: 0, lsu: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", DEC_NUMBER_ONE));
  // two
  assert_eq!("DecNumber { digits: 1, exponent: 0, bits: 0, lsu: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", n!(2)));
  assert_eq!("DecNumber { digits: 1, exponent: 0, bits: 0, lsu: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", DEC_NUMBER_TWO));
  // ten
  assert_eq!("DecNumber { digits: 2, exponent: 0, bits: 0, lsu: [10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", n!(10)));
  assert_eq!("DecNumber { digits: 2, exponent: 0, bits: 0, lsu: [10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", DEC_NUMBER_TEN));
  // hundred
  assert_eq!("DecNumber { digits: 3, exponent: 0, bits: 0, lsu: [100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", n!(100)));
  assert_eq!("DecNumber { digits: 3, exponent: 0, bits: 0, lsu: [100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", DEC_NUMBER_HUNDRED));
  // thousand
  assert_eq!("DecNumber { digits: 4, exponent: 0, bits: 0, lsu: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", n!(1000)));
  assert_eq!("DecNumber { digits: 4, exponent: 0, bits: 0, lsu: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", DEC_NUMBER_THOUSAND));
  // million
  assert_eq!("DecNumber { digits: 7, exponent: 0, bits: 0, lsu: [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", n!(1000000)));
  assert_eq!("DecNumber { digits: 7, exponent: 0, bits: 0, lsu: [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", DEC_NUMBER_MILLION));
  // billion
  assert_eq!("DecNumber { digits: 10, exponent: 0, bits: 0, lsu: [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", n!(1000000000)));
  assert_eq!("DecNumber { digits: 10, exponent: 0, bits: 0, lsu: [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0] }", format!("{:?}", DEC_NUMBER_BILLION));
}

#[test]
#[allow(clippy::clone_on_copy)]
fn test_dec_number_copy_clone() {
  let n1 = n!(1.2345);
  let n2 = n1;
  let n3 = n1.clone();
  assert_eq!("1.2345", s!(n2));
  assert_eq!("1.2345", s!(n3));
}

#[test]
fn test_dec_number_divide() {
  assert_eq!("0.25", s!(dec_number_divide(&n!(1), &n!(4), c!())));
}

#[test]
fn test_dec_number_exp() {
  assert_eq!("7.389056098930650227230427460575008", s!(dec_number_exp(&n!(2), c!())));
}

#[test]
fn test_dec_number_from_i32() {
  assert_eq!("-12", s!(dec_number_from_i32(-12)));
}

#[test]
fn test_dec_number_from_u32() {
  assert_eq!("12", s!(dec_number_from_u32(12)));
}

#[test]
fn test_dec_number_is_negative() {
  assert!(dec_number_is_negative(&n!(-1)));
  assert!(!dec_number_is_negative(&n!(1)));
  assert!(!dec_number_is_negative(&n!(NaN)));
}

#[test]
fn test_dec_number_is_zero() {
  assert!(dec_number_is_zero(&n!(0)));
  assert!(!dec_number_is_zero(&n!(0.1)));
  assert!(!dec_number_is_zero(&n!(-0.1)));
}

#[test]
fn test_dec_number_ln() {
  assert_eq!("0.6931471805599453094172321214581766", s!(dec_number_ln(&n!(2), c!())));
}

#[test]
fn test_dec_number_minus() {
  assert_eq!("0", s!(dec_number_minus(&n!(-0), c!())));
  assert_eq!("0", s!(dec_number_minus(&n!(0), c!())));
  assert_eq!("1.1", s!(dec_number_minus(&n!(-1.1), c!())));
  assert_eq!("-1.1", s!(dec_number_minus(&n!(1.1), c!())));
}

#[test]
fn test_dec_number_multiply() {
  assert_eq!("4.4", s!(dec_number_multiply(&n!(1.1), &n!(4), c!())));
}

#[test]
fn test_dec_number_plus() {
  assert_eq!("0", s!(dec_number_plus(&n!(-0), c!())));
  assert_eq!("0", s!(dec_number_plus(&n!(0), c!())));
  assert_eq!("-1.1", s!(dec_number_plus(&n!(-1.1), c!())));
  assert_eq!("1.1", s!(dec_number_plus(&n!(1.1), c!())));
}

#[test]
fn test_dec_number_power() {
  assert_eq!("8", s!(dec_number_power(&n!(2), &n!(3), c!())));
}

#[test]
fn test_dec_number_quantize() {
  assert_eq!("123.5", s!(dec_number_quantize(&n!(123.456), &n!(10.0), c!())));
}

#[test]
fn test_dec_number_reduce() {
  assert_eq!("1.2345678E+11", s!(dec_number_reduce(&n!(12345678E+4), c!())));
}

#[test]
fn test_dec_number_rescale() {
  assert_eq!("123.5", s!(dec_number_rescale(&n!(123.456), &n!(-1), c!())));
}

#[test]
fn test_dec_number_scale_b() {
  assert_eq!("1234.5678", s!(dec_number_scale_b(&n!(12345678), &n!(-4), c!())));
}

#[test]
fn test_dec_number_square_root() {
  assert_eq!(
    "1.414213562373095048801688724209698",
    s!(dec_number_square_root(&n!(2), c!()))
  );
}

#[test]
fn test_dec_number_subtract() {
  assert_eq!("1.000", s!(dec_number_subtract(&n!(1.123), &n!(0.123), c!())));
}

#[test]
fn test_dec_number_zero() {
  let mut n = n!(-0.75);
  dec_number_zero(&mut n);
  assert_eq!("0", s!(n));
}
