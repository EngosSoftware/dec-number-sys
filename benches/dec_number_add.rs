#![feature(test)]

extern crate test;

use dec_number_sys::*;
use test::Bencher;

#[bench]
fn bench_dec_number_add_0001(b: &mut Bencher) {
  let dc = &mut dec_context_128();
  let dnl = &dec_number_from_string("0.1", dc);
  let dnr = &dec_number_from_string("0.2", dc);
  b.iter(|| {
    let _ = dec_number_add(dnl, dnr, dc);
  });
}

#[bench]
fn bench_dec_number_add_0002(b: &mut Bencher) {
  let dc = &mut dec_context_128();
  let dnl = &dec_number_from_string("0.111111", dc);
  let dnr = &dec_number_from_string("0.222", dc);
  b.iter(|| {
    let _ = dec_number_add(dnl, dnr, dc);
  });
}
