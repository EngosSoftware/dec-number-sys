use dec_number_sys::bcd;

#[test]

fn dec_common_bcd() {
  assert_eq!([9, 8, 7, 6, 5, 4, 3, 2, 1, 0], bcd(9876543210).as_slice());
  assert_eq!(
    [
      3, 4, 0, 2, 8, 2, 3, 6, 6, 9, 2, 0, 9, 3, 8, 4, 6, 3, 4, 6, 3, 3, 7, 4, 6, 0, 7, 4, 3, 1, 7, 6, 8, 2, 1, 1, 4, 5,
      5
    ],
    bcd(u128::MAX).as_slice()
  );
  assert_eq!(
    [1, 8, 4, 4, 6, 7, 4, 4, 0, 7, 3, 7, 0, 9, 5, 5, 1, 6, 1, 5],
    bcd(u64::MAX.into()).as_slice()
  );
  assert_eq!(39, bcd(u128::MAX).as_slice().len());
}
